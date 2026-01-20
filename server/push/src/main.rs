//! MePassa Push Notification Server
//!
//! Handles push notifications for FCM (Firebase Cloud Messaging) and APNs (Apple Push Notification Service).
//!
//! # Architecture
//! - Axum web framework for REST API
//! - PostgreSQL for storing device tokens
//! - FCM for sending Android notifications
//! - APNs support coming in future (FASE 13 - iOS)

mod api;
mod fcm;

use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub fcm_client: Arc<fcm::FcmClient>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Setup logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mepassa_push=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 MePassa Push Notification Server starting...");

    // Get configuration from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let fcm_server_key = std::env::var("FCM_SERVER_KEY")
        .expect("FCM_SERVER_KEY must be set");
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Connect to database
    tracing::info!("📦 Connecting to database...");
    let db_pool = sqlx::PgPool::connect(&database_url).await?;
    tracing::info!("✅ Database connected");

    // Initialize FCM client
    tracing::info!("🔥 Initializing FCM client...");
    let fcm_client = Arc::new(fcm::FcmClient::new(fcm_server_key));
    tracing::info!("✅ FCM client ready");

    // Create app state
    let state = AppState {
        db_pool,
        fcm_client,
    };

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/register", post(api::register::handle))
        .route("/api/v1/send", post(api::send::handle))
        .route("/api/v1/unregister", delete(api::unregister::handle))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🎧 Push server listening on {}", addr);
    tracing::info!("📡 Ready to handle push notifications!");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}
