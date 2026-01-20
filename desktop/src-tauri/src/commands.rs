use mepassa_core::api::client::MePassaClient;
use std::sync::{Arc, Mutex};
use tauri::State;

// Global client state
type ClientState = Arc<Mutex<Option<MePassaClient>>>;

#[tauri::command]
pub async fn init_client(
    state: State<'_, ClientState>,
    data_dir: String,
) -> Result<String, String> {
    tracing::info!("Initializing MePassa client with data_dir: {}", data_dir);

    let client = MePassaClient::new(&data_dir).map_err(|e| e.to_string())?;
    let peer_id = client.local_peer_id().map_err(|e| e.to_string())?;

    let mut client_guard = state.lock().map_err(|e| e.to_string())?;
    *client_guard = Some(client);

    tracing::info!("Client initialized with peer_id: {}", peer_id);
    Ok(peer_id)
}

#[tauri::command]
pub async fn get_local_peer_id(state: State<'_, ClientState>) -> Result<String, String> {
    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client.local_peer_id().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn listen_on(
    state: State<'_, ClientState>,
    multiaddr: String,
) -> Result<(), String> {
    tracing::info!("Listening on: {}", multiaddr);

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client.listen_on(&multiaddr).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn connect_to_peer(
    state: State<'_, ClientState>,
    peer_id: String,
    multiaddr: String,
) -> Result<(), String> {
    tracing::info!("Connecting to peer {} at {}", peer_id, multiaddr);

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client
        .connect_to_peer(&peer_id, &multiaddr)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_text_message(
    state: State<'_, ClientState>,
    to_peer_id: String,
    content: String,
) -> Result<String, String> {
    tracing::info!("Sending message to peer: {}", to_peer_id);

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client
        .send_text_message(&to_peer_id, &content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_conversation_messages(
    state: State<'_, ClientState>,
    peer_id: String,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<serde_json::Value>, String> {
    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    let messages = client
        .get_conversation_messages(&peer_id, limit.unwrap_or(50), offset.unwrap_or(0))
        .map_err(|e| e.to_string())?;

    // Convert messages to JSON
    let json_messages: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "id": m.id,
                "from_peer_id": m.from_peer_id,
                "to_peer_id": m.to_peer_id,
                "content": m.content,
                "timestamp": m.timestamp.timestamp(),
                "is_read": m.is_read,
            })
        })
        .collect();

    Ok(json_messages)
}

#[tauri::command]
pub async fn list_conversations(
    state: State<'_, ClientState>,
) -> Result<Vec<serde_json::Value>, String> {
    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    let conversations = client.list_conversations().map_err(|e| e.to_string())?;

    // Convert conversations to JSON
    let json_conversations: Vec<serde_json::Value> = conversations
        .iter()
        .map(|c| {
            serde_json::json!({
                "peer_id": c.peer_id,
                "last_message": c.last_message,
                "last_message_timestamp": c.last_message_timestamp.timestamp(),
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
    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    let messages = client
        .search_messages(&query, limit.unwrap_or(50))
        .map_err(|e| e.to_string())?;

    // Convert messages to JSON
    let json_messages: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "id": m.id,
                "from_peer_id": m.from_peer_id,
                "to_peer_id": m.to_peer_id,
                "content": m.content,
                "timestamp": m.timestamp.timestamp(),
                "is_read": m.is_read,
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
    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client
        .mark_conversation_read(&peer_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_connected_peers_count(state: State<'_, ClientState>) -> Result<u32, String> {
    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client.connected_peers_count().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn bootstrap(state: State<'_, ClientState>) -> Result<(), String> {
    tracing::info!("Bootstrapping DHT...");

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client.bootstrap().map_err(|e| e.to_string())
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

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client.start_call(&to_peer_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn accept_call(
    state: State<'_, ClientState>,
    call_id: String,
) -> Result<(), String> {
    tracing::info!("Accepting call: {}", call_id);

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client.accept_call(&call_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reject_call(
    state: State<'_, ClientState>,
    call_id: String,
    reason: Option<String>,
) -> Result<(), String> {
    tracing::info!("Rejecting call: {} (reason: {:?})", call_id, reason);

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client
        .reject_call(&call_id, reason.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn hangup_call(
    state: State<'_, ClientState>,
    call_id: String,
) -> Result<(), String> {
    tracing::info!("Hanging up call: {}", call_id);

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client.hangup_call(&call_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_mute(
    state: State<'_, ClientState>,
    call_id: String,
) -> Result<bool, String> {
    tracing::info!("Toggling mute for call: {}", call_id);

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client.toggle_mute(&call_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_speakerphone(
    state: State<'_, ClientState>,
    call_id: String,
) -> Result<bool, String> {
    tracing::info!("Toggling speakerphone for call: {}", call_id);

    let client_guard = state.lock().map_err(|e| e.to_string())?;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| "Client not initialized".to_string())?;

    client
        .toggle_speakerphone(&call_id)
        .map_err(|e| e.to_string())
}
