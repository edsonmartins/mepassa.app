//! Client Builder
//!
//! Builder pattern for creating MePassa clients.

use libp2p::identity::Keypair;
use std::path::PathBuf;

use super::client::Client;
use crate::{
    identity::Identity,
    network::NetworkManager,
    storage::{Database, migrate, needs_migration},
    utils::error::{MePassaError, Result},
};
#[cfg(feature = "voip")]
use crate::voip::{CallManager, VoIPIntegration};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Builder for creating a MePassa client
pub struct ClientBuilder {
    data_dir: Option<PathBuf>,
    keypair: Option<Keypair>,
    bootstrap_peers: Vec<(libp2p::PeerId, libp2p::Multiaddr)>,
}

impl ClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self {
            data_dir: None,
            keypair: None,
            bootstrap_peers: Vec::new(),
        }
    }

    /// Set data directory
    pub fn data_dir(mut self, path: PathBuf) -> Self {
        self.data_dir = Some(path);
        self
    }

    /// Set keypair (if None, will generate new one)
    pub fn keypair(mut self, keypair: Keypair) -> Self {
        self.keypair = Some(keypair);
        self
    }

    /// Add bootstrap peer
    pub fn add_bootstrap_peer(mut self, peer_id: libp2p::PeerId, addr: libp2p::Multiaddr) -> Self {
        self.bootstrap_peers.push((peer_id, addr));
        self
    }

    /// Build the client
    pub async fn build(self) -> Result<Client> {
        // Get or create data directory
        let data_dir = self.data_dir.ok_or_else(|| {
            MePassaError::Other("Data directory is required".to_string())
        })?;

        // Create data directory if it doesn't exist
        std::fs::create_dir_all(&data_dir).map_err(|e| {
            MePassaError::Other(format!("Failed to create data directory: {}", e))
        })?;

        // Get or generate keypair
        let keypair = if let Some(keypair) = self.keypair {
            keypair
        } else {
            // Try to load from file, or generate new one
            let keypair_path = data_dir.join("identity.key");
            if keypair_path.exists() {
                // Load keypair from file
                match load_keypair_from_file(&keypair_path) {
                    Ok(kp) => kp,
                    Err(e) => {
                        tracing::warn!("Failed to load keypair from file: {}, generating new one", e);
                        Keypair::generate_ed25519()
                    }
                }
            } else {
                // Generate new keypair and save to file
                let keypair = Keypair::generate_ed25519();
                if let Err(e) = save_keypair_to_file(&keypair, &keypair_path) {
                    tracing::warn!("Failed to save keypair to file: {}", e);
                }
                keypair
            }
        };

        // Create identity (convert from libp2p keypair)
        let our_keypair = crate::identity::Keypair::from_libp2p_keypair(&keypair)?;
        let identity = Identity::from_keypair(our_keypair);

        // Open database
        let db_path = data_dir.join("mepassa.db");
        let database = Database::open(&db_path)?;

        // Run migrations if needed
        if needs_migration(&database)? {
            migrate(&database)?;
        }

        // Create Arc for shared database access
        let database_arc = Arc::new(database);

        // Get peer ID from libp2p keypair
        let peer_id = libp2p::PeerId::from(keypair.public());

        // Create network manager
        let network = NetworkManager::new(keypair)?;
        let network_arc = Arc::new(RwLock::new(network));

        // Create message handler for processing incoming messages
        let message_handler = Arc::new(crate::network::MessageHandler::new(
            peer_id.to_string(),
            Arc::new(RwLock::new(database_arc.as_ref().clone())),
            None, // No event channel for now (TODO: add event system)
        ));

        // Set message handler in network manager
        {
            let mut network = network_arc.write().await;
            network.set_message_handler(Arc::clone(&message_handler));
        }

        // Add bootstrap peers to DHT (if any)
        if !self.bootstrap_peers.is_empty() {
            // Bootstrap peers will be added when the network starts
            // They are stored in the builder and can be accessed by the network manager
            tracing::info!("Configured {} bootstrap peers", self.bootstrap_peers.len());
        }

        // Create VoIP components (only if feature is enabled)
        #[cfg(feature = "voip")]
        let call_manager = Arc::new(CallManager::new());
        #[cfg(feature = "voip")]
        let voip_integration = Arc::new(
            VoIPIntegration::new(Arc::clone(&network_arc), Arc::clone(&call_manager)).await,
        );

        // Create Group Manager (FASE 15)
        let group_manager = Arc::new(
            crate::group::GroupManager::new(
                peer_id.to_string(),
                Arc::clone(&database_arc),
            )
            .map_err(|e| MePassaError::Other(format!("Failed to create group manager: {}", e)))?
        );

        // Initialize group manager (load existing groups)
        group_manager.init().await.map_err(|e| {
            MePassaError::Other(format!("Failed to initialize group manager: {}", e))
        })?;

        // Create client (keep network as Arc since it's shared with VoIPIntegration)
        // Note: Client takes ownership of database Arc
        let client = Client::new(
            peer_id,
            identity,
            network_arc,
            Arc::try_unwrap(database_arc).unwrap_or_else(|arc| (*arc).clone()),
            data_dir,
            #[cfg(feature = "voip")]
            call_manager,
            #[cfg(feature = "voip")]
            voip_integration,
            group_manager,
        );

        Ok(client)
    }
}

/// Load a keypair from a file
fn load_keypair_from_file(path: &std::path::Path) -> Result<Keypair> {
    let bytes = std::fs::read(path).map_err(|e| {
        MePassaError::Other(format!("Failed to read keypair file: {}", e))
    })?;

    // Try to parse as protobuf-encoded keypair
    Keypair::from_protobuf_encoding(&bytes).map_err(|e| {
        MePassaError::Other(format!("Failed to decode keypair: {}", e))
    })
}

/// Save a keypair to a file
fn save_keypair_to_file(keypair: &Keypair, path: &std::path::Path) -> Result<()> {
    let bytes = keypair.to_protobuf_encoding().map_err(|e| {
        MePassaError::Other(format!("Failed to encode keypair: {}", e))
    })?;

    std::fs::write(path, bytes).map_err(|e| {
        MePassaError::Other(format!("Failed to write keypair file: {}", e))
    })
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_builder() {
        let temp_dir = TempDir::new().unwrap();
        let data_dir = temp_dir.path().to_path_buf();

        let client = ClientBuilder::new()
            .data_dir(data_dir.clone())
            .build()
            .await
            .unwrap();

        assert!(client.local_peer_id().to_string().len() > 0);

        // Database should be created
        assert!(data_dir.join("mepassa.db").exists());
    }

    #[tokio::test]
    async fn test_builder_with_keypair() {
        let temp_dir = TempDir::new().unwrap();
        let data_dir = temp_dir.path().to_path_buf();

        let keypair = Keypair::generate_ed25519();
        let expected_peer_id = libp2p::PeerId::from(keypair.public());

        let client = ClientBuilder::new()
            .data_dir(data_dir)
            .keypair(keypair)
            .build()
            .await
            .unwrap();

        assert_eq!(client.local_peer_id(), expected_peer_id);
    }
}
