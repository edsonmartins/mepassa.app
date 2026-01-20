//! FFI Client implementation using UniFFI (channel-based architecture)
//!
//! This module provides a thread-safe FFI wrapper around the MePassa Client.
//! Since libp2p::Swarm is !Sync by design, we use a channel-based architecture
//! where the Client runs in a dedicated tokio task and receives commands via channels.

use std::path::PathBuf;
use std::sync::OnceLock;
use tokio::sync::{mpsc, oneshot};

use super::types::{FfiConversation, FfiMessage, MePassaFfiError};
use crate::api::{Client, ClientBuilder};

use std::thread;
use tokio::task::LocalSet;

// Global tokio runtime with LocalSet for !Send futures
static RUNTIME: OnceLock<std::sync::Arc<tokio::runtime::Runtime>> = OnceLock::new();

fn runtime() -> &'static std::sync::Arc<tokio::runtime::Runtime> {
    RUNTIME.get_or_init(|| {
        std::sync::Arc::new(tokio::runtime::Runtime::new().expect("Failed to create tokio runtime"))
    })
}

// Global client handle (initialized once)
static CLIENT_HANDLE: OnceLock<ClientHandle> = OnceLock::new();

/// Handle to communicate with the Client running in a dedicated task
struct ClientHandle {
    sender: mpsc::UnboundedSender<ClientCommand>,
}

/// Commands that can be sent to the Client
enum ClientCommand {
    LocalPeerId {
        response: oneshot::Sender<String>,
    },
    ListenOn {
        multiaddr: libp2p::Multiaddr,
        response: oneshot::Sender<Result<(), MePassaFfiError>>,
    },
    ConnectToPeer {
        peer_id: libp2p::PeerId,
        multiaddr: libp2p::Multiaddr,
        response: oneshot::Sender<Result<(), MePassaFfiError>>,
    },
    SendTextMessage {
        to: libp2p::PeerId,
        content: String,
        response: oneshot::Sender<Result<String, MePassaFfiError>>,
    },
    GetConversationMessages {
        peer_id: String,
        limit: Option<usize>,
        offset: Option<usize>,
        response: oneshot::Sender<Result<Vec<FfiMessage>, MePassaFfiError>>,
    },
    ListConversations {
        response: oneshot::Sender<Result<Vec<FfiConversation>, MePassaFfiError>>,
    },
    SearchMessages {
        query: String,
        limit: Option<usize>,
        response: oneshot::Sender<Result<Vec<FfiMessage>, MePassaFfiError>>,
    },
    MarkConversationRead {
        peer_id: String,
        response: oneshot::Sender<Result<(), MePassaFfiError>>,
    },
    ConnectedPeersCount {
        response: oneshot::Sender<Result<u32, MePassaFfiError>>,
    },
    Bootstrap {
        response: oneshot::Sender<Result<(), MePassaFfiError>>,
    },
}

/// Run the client task (processes commands)
async fn run_client_task(mut receiver: mpsc::UnboundedReceiver<ClientCommand>, client: Client) {
    while let Some(cmd) = receiver.recv().await {
        match cmd {
            ClientCommand::LocalPeerId { response } => {
                let _ = response.send(client.local_peer_id().to_string());
            }
            ClientCommand::ListenOn {
                multiaddr,
                response,
            } => {
                let result = client
                    .listen_on(multiaddr)
                    .await
                    .map_err(|e| e.into());
                let _ = response.send(result);
            }
            ClientCommand::ConnectToPeer {
                peer_id,
                multiaddr,
                response,
            } => {
                let result = client
                    .connect_to_peer(peer_id, multiaddr)
                    .await
                    .map_err(|e| e.into());
                let _ = response.send(result);
            }
            ClientCommand::SendTextMessage { to, content, response } => {
                let result = client
                    .send_text_message(to, content)
                    .await
                    .map_err(|e| e.into());
                let _ = response.send(result);
            }
            ClientCommand::GetConversationMessages {
                peer_id,
                limit,
                offset,
                response,
            } => {
                let result = client
                    .get_conversation_messages(&peer_id, limit, offset)
                    .map(|messages| messages.into_iter().map(FfiMessage::from).collect())
                    .map_err(|e| e.into());
                let _ = response.send(result);
            }
            ClientCommand::ListConversations { response } => {
                let result = client
                    .list_conversations()
                    .map(|convs| convs.into_iter().map(FfiConversation::from).collect())
                    .map_err(|e| e.into());
                let _ = response.send(result);
            }
            ClientCommand::SearchMessages {
                query,
                limit,
                response,
            } => {
                let result = client
                    .search_messages(&query, limit)
                    .map(|messages| messages.into_iter().map(FfiMessage::from).collect())
                    .map_err(|e| e.into());
                let _ = response.send(result);
            }
            ClientCommand::MarkConversationRead { peer_id, response } => {
                let result = client
                    .mark_conversation_read(&peer_id)
                    .map_err(|e| e.into());
                let _ = response.send(result);
            }
            ClientCommand::ConnectedPeersCount { response } => {
                let result = Ok(client.connected_peers_count().await as u32);
                let _ = response.send(result);
            }
            ClientCommand::Bootstrap { response } => {
                let result = client.bootstrap().await.map_err(|e| e.into());
                let _ = response.send(result);
            }
        }
    }
}

/// MePassa client (exposed via interface pattern)
pub struct MePassaClient {
    data_dir: String,
}

impl MePassaClient {
    /// Create new client and initialize the global client task
    pub fn new(data_dir: String) -> Result<Self, MePassaFfiError> {
        // Initialize the client task if not already done
        CLIENT_HANDLE.get_or_init(|| {
            let (sender, receiver) = mpsc::unbounded_channel();
            let data_dir_clone = data_dir.clone();

            // Spawn a dedicated thread with LocalSet for !Send Client
            thread::spawn(move || {
                let rt = runtime();
                let local = LocalSet::new();

                // Build and run the client task
                local.block_on(rt, async move {
                    let client = ClientBuilder::new()
                        .data_dir(PathBuf::from(&data_dir_clone))
                        .build()
                        .await
                        .expect("Failed to build client");

                    run_client_task(receiver, client).await;
                });
            });

            ClientHandle { sender }
        });

        Ok(Self { data_dir })
    }

    /// Get the client handle
    fn handle(&self) -> &ClientHandle {
        CLIENT_HANDLE.get().expect("Client not initialized")
    }

    /// Get local peer ID
    pub fn local_peer_id(&self) -> Result<String, MePassaFfiError> {
        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::LocalPeerId { response: tx })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        runtime().block_on(rx).map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })
    }

    /// Start listening on an address
    pub async fn listen_on(&self, multiaddr: String) -> Result<(), MePassaFfiError> {
        let addr: libp2p::Multiaddr = multiaddr.parse().map_err(|_| MePassaFfiError::Network {
            message: "Invalid multiaddr".to_string(),
        })?;

        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::ListenOn {
                multiaddr: addr,
                response: tx,
            })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        rx.await.map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }

    /// Connect to a peer
    pub async fn connect_to_peer(
        &self,
        peer_id: String,
        multiaddr: String,
    ) -> Result<(), MePassaFfiError> {
        let peer_id: libp2p::PeerId = peer_id.parse().map_err(|_| MePassaFfiError::Network {
            message: "Invalid peer ID".to_string(),
        })?;

        let addr: libp2p::Multiaddr = multiaddr.parse().map_err(|_| MePassaFfiError::Network {
            message: "Invalid multiaddr".to_string(),
        })?;

        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::ConnectToPeer {
                peer_id,
                multiaddr: addr,
                response: tx,
            })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        rx.await.map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }

    /// Send a text message
    pub async fn send_text_message(
        &self,
        to_peer_id: String,
        content: String,
    ) -> Result<String, MePassaFfiError> {
        let to: libp2p::PeerId = to_peer_id.parse().map_err(|_| MePassaFfiError::Network {
            message: "Invalid peer ID".to_string(),
        })?;

        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::SendTextMessage {
                to,
                content,
                response: tx,
            })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        rx.await.map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }

    /// Get messages for a conversation
    pub fn get_conversation_messages(
        &self,
        peer_id: String,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<FfiMessage>, MePassaFfiError> {
        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::GetConversationMessages {
                peer_id,
                limit: limit.map(|l| l as usize),
                offset: offset.map(|o| o as usize),
                response: tx,
            })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        runtime().block_on(rx).map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }

    /// List all conversations
    pub fn list_conversations(&self) -> Result<Vec<FfiConversation>, MePassaFfiError> {
        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::ListConversations { response: tx })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        runtime().block_on(rx).map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }

    /// Search messages
    pub fn search_messages(
        &self,
        query: String,
        limit: Option<u32>,
    ) -> Result<Vec<FfiMessage>, MePassaFfiError> {
        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::SearchMessages {
                query,
                limit: limit.map(|l| l as usize),
                response: tx,
            })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        runtime().block_on(rx).map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }

    /// Mark conversation as read
    pub fn mark_conversation_read(&self, peer_id: String) -> Result<(), MePassaFfiError> {
        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::MarkConversationRead {
                peer_id,
                response: tx,
            })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        runtime().block_on(rx).map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }

    /// Get connected peers count
    pub async fn connected_peers_count(&self) -> Result<u32, MePassaFfiError> {
        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::ConnectedPeersCount { response: tx })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        rx.await.map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }

    /// Bootstrap DHT
    pub async fn bootstrap(&self) -> Result<(), MePassaFfiError> {
        let (tx, rx) = oneshot::channel();
        self.handle()
            .sender
            .send(ClientCommand::Bootstrap { response: tx })
            .map_err(|_| MePassaFfiError::Other {
                message: "Failed to send command".to_string(),
            })?;

        rx.await.map_err(|_| MePassaFfiError::Other {
            message: "Failed to receive response".to_string(),
        })?
    }
}
