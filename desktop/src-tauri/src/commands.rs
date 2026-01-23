use mepassa_core::ffi::MePassaClient;
use std::sync::{Arc, Mutex};
use tauri::State;
use tauri_plugin_notification::NotificationExt;

// Global client state - use Arc to allow cloning the handle
type ClientState = Arc<Mutex<Option<Arc<MePassaClient>>>>;

#[tauri::command]
pub async fn init_client(
    state: State<'_, ClientState>,
    data_dir: String,
) -> Result<String, String> {
    tracing::info!("🔵 init_client CALLED with data_dir: {}", data_dir);

    // MePassaClient::new() is synchronous, not async
    tracing::info!("🔵 Creating MePassaClient...");
    let client = Arc::new(MePassaClient::new(data_dir.clone()).map_err(|e| {
        tracing::error!("❌ Failed to create MePassaClient: {}", e);
        e.to_string()
    })?);

    tracing::info!("🔵 Getting local peer ID...");
    let client_clone = client.clone();
    let peer_id = tokio::task::spawn_blocking(move || {
        client_clone.local_peer_id()
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
    .map_err(|e| {
        tracing::error!("❌ Failed to get local peer ID: {}", e);
        e.to_string()
    })?;

    tracing::info!("🔵 Storing client in state...");
    let mut client_guard = state.lock().map_err(|e| {
        tracing::error!("❌ Failed to lock state: {}", e);
        e.to_string()
    })?;
    *client_guard = Some(client);

    tracing::info!("✅ Client initialized successfully with peer_id: {}", peer_id);
    Ok(peer_id)
}

#[tauri::command]
pub async fn get_local_peer_id(state: State<'_, ClientState>) -> Result<String, String> {
    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    // local_peer_id() is synchronous
    client.local_peer_id().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn listen_on(
    state: State<'_, ClientState>,
    multiaddr: String,
) -> Result<(), String> {
    tracing::info!("Listening on: {}", multiaddr);

    // Clone Arc to avoid holding MutexGuard across await
    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    // listen_on() is async and takes owned String
    client.listen_on(multiaddr).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn connect_to_peer(
    state: State<'_, ClientState>,
    peer_id: String,
    multiaddr: String,
) -> Result<(), String> {
    tracing::info!("Connecting to peer {} at {}", peer_id, multiaddr);

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client
        .connect_to_peer(peer_id, multiaddr)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_text_message(
    state: State<'_, ClientState>,
    to_peer_id: String,
    content: String,
) -> Result<String, String> {
    tracing::info!("Sending message to peer: {}", to_peer_id);

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client
        .send_text_message(to_peer_id, content)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_conversation_messages(
    state: State<'_, ClientState>,
    peer_id: String,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<serde_json::Value>, String> {
    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    // get_conversation_messages() is synchronous
    let messages = client
        .get_conversation_messages(peer_id, limit, offset)
        .map_err(|e| e.to_string())?;

    // Convert messages to JSON
    let json_messages: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "id": m.message_id,
                "sender_peer_id": m.sender_peer_id,
                "recipient_peer_id": m.recipient_peer_id,
                "content": m.content_plaintext,
                "created_at": m.created_at,
                "status": format!("{:?}", m.status),
            })
        })
        .collect();

    Ok(json_messages)
}

#[tauri::command]
pub async fn list_conversations(
    state: State<'_, ClientState>,
) -> Result<Vec<serde_json::Value>, String> {
    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    // list_conversations() is synchronous
    let conversations = client.list_conversations().map_err(|e| e.to_string())?;

    // Convert conversations to JSON
    let json_conversations: Vec<serde_json::Value> = conversations
        .iter()
        .map(|c| {
            serde_json::json!({
                "id": c.id,
                "peer_id": c.peer_id,
                "display_name": c.display_name,
                "last_message_id": c.last_message_id,
                "last_message_at": c.last_message_at,
                "unread_count": c.unread_count,
            })
        })
        .collect();

    Ok(json_conversations)
}

#[tauri::command]
pub async fn search_messages(
    state: State<'_, ClientState>,
    query: String,
    limit: Option<u32>,
) -> Result<Vec<serde_json::Value>, String> {
    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    // search_messages() is synchronous
    let messages = client
        .search_messages(query, limit)
        .map_err(|e| e.to_string())?;

    // Convert messages to JSON
    let json_messages: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "id": m.message_id,
                "sender_peer_id": m.sender_peer_id,
                "recipient_peer_id": m.recipient_peer_id,
                "content": m.content_plaintext,
                "created_at": m.created_at,
                "status": format!("{:?}", m.status),
            })
        })
        .collect();

    Ok(json_messages)
}

#[tauri::command]
pub async fn mark_conversation_read(
    state: State<'_, ClientState>,
    peer_id: String,
) -> Result<(), String> {
    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    // mark_conversation_read() is synchronous
    client
        .mark_conversation_read(peer_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_connected_peers_count(state: State<'_, ClientState>) -> Result<u32, String> {
    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    // connected_peers_count() is async and returns u32
    client.connected_peers_count().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn bootstrap(state: State<'_, ClientState>) -> Result<(), String> {
    tracing::info!("Bootstrapping DHT...");

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client.bootstrap().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn show_notification(
    app: tauri::AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    tracing::info!("Showing notification: {} - {}", title, body);

    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// VoIP Commands (FASE 12)
// ============================================================================

#[tauri::command]
pub async fn start_call(
    state: State<'_, ClientState>,
    to_peer_id: String,
) -> Result<String, String> {
    tracing::info!("Starting call to peer: {}", to_peer_id);

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client.start_call(to_peer_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn accept_call(
    state: State<'_, ClientState>,
    call_id: String,
) -> Result<(), String> {
    tracing::info!("Accepting call: {}", call_id);

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client.accept_call(call_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reject_call(
    state: State<'_, ClientState>,
    call_id: String,
    reason: Option<String>,
) -> Result<(), String> {
    tracing::info!("Rejecting call: {} (reason: {:?})", call_id, reason);

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client
        .reject_call(call_id, reason)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn hangup_call(
    state: State<'_, ClientState>,
    call_id: String,
) -> Result<(), String> {
    tracing::info!("Hanging up call: {}", call_id);

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client.hangup_call(call_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_mute(
    state: State<'_, ClientState>,
    call_id: String,
) -> Result<(), String> {
    tracing::info!("Toggling mute for call: {}", call_id);

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client.toggle_mute(call_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_speakerphone(
    state: State<'_, ClientState>,
    call_id: String,
) -> Result<(), String> {
    tracing::info!("Toggling speakerphone for call: {}", call_id);

    let client = {
        let client_guard = state.lock().map_err(|e| e.to_string())?;
        client_guard
            .as_ref()
            .ok_or_else(|| "Client not initialized".to_string())?
            .clone()
    };

    client
        .toggle_speakerphone(call_id)
        .await
        .map_err(|e| e.to_string())
}
