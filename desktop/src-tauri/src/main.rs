// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::sync::{Arc, Mutex};
use tracing_subscriber;

fn main() {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("mepassa_desktop=debug,mepassa_core=debug")
        .init();

    // Initialize client state
    let client_state: Arc<Mutex<Option<Arc<mepassa_core::ffi::MePassaClient>>>> =
        Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(client_state)
        .invoke_handler(tauri::generate_handler![
            commands::init_client,
            commands::get_local_peer_id,
            commands::listen_on,
            commands::connect_to_peer,
            commands::send_text_message,
            commands::get_conversation_messages,
            commands::list_conversations,
            commands::search_messages,
            commands::mark_conversation_read,
            commands::get_connected_peers_count,
            commands::bootstrap,
            commands::show_notification,
            // VoIP commands (FASE 12)
            commands::start_call,
            commands::accept_call,
            commands::reject_call,
            commands::hangup_call,
            commands::toggle_mute,
            commands::toggle_speakerphone,
        ])
        .setup(|app| {
            // Setup app-specific initialization here
            tracing::info!("MePassa Desktop starting...");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
