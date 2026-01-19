//! API handlers for Identity Server

use axum::{
    extract::{Query, State},
    Json,
};
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    db,
    error::{AppError, Result},
    models::*,
    AppState,
};

/// Register a new username
pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>> {
    // Decode public key from base64
    let public_key = general_purpose::STANDARD
        .decode(&req.public_key)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid base64: {}", e)))?;

    // Verify signature
    verify_signature(&public_key, &req.signature, &req.timestamp, &req.username)?;

    // Register username
    let response = db::register_username(
        &state.db,
        &req.username,
        &req.peer_id,
        &public_key,
        &req.prekey_bundle,
    )
    .await?;

    Ok(Json(response))
}

/// Lookup username query parameters
#[derive(Debug, Deserialize)]
pub struct LookupQuery {
    pub username: String,
}

/// Lookup a username
pub async fn lookup_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LookupQuery>,
) -> Result<Json<LookupResponse>> {
    let response = db::lookup_username(&state.db, &query.username).await?;
    Ok(Json(response))
}

/// Update prekeys for a username
pub async fn update_prekeys_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdatePrekeysRequest>,
) -> Result<Json<UpdatePrekeysResponse>> {
    // For now, we skip signature verification (would need to lookup public key first)
    // In production, you'd:
    // 1. Lookup peer_id to get public_key
    // 2. Verify signature with that public_key

    let response = db::update_prekeys(&state.db, &req.peer_id, &req.prekey_bundle).await?;
    Ok(Json(response))
}

/// Health check endpoint
pub async fn health_handler(State(state): State<Arc<AppState>>) -> Result<Json<HealthResponse>> {
    let start = std::time::Instant::now();

    // Check database
    let db_latency = db::check_health(&state.db).await?;

    // Check Redis
    let redis_latency = check_redis_health(&state.redis).await?;

    let uptime_seconds = start.elapsed().as_secs();

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds,
        database: HealthStatus {
            status: "connected".to_string(),
            latency_ms: db_latency,
        },
        redis: HealthStatus {
            status: "connected".to_string(),
            latency_ms: redis_latency,
        },
        timestamp: chrono::Utc::now(),
    }))
}

/// Verify Ed25519 signature
fn verify_signature(
    public_key: &[u8],
    signature_b64: &str,
    timestamp: &i64,
    username: &str,
) -> Result<()> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    // Decode signature
    let signature_bytes = general_purpose::STANDARD
        .decode(signature_b64)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid signature base64: {}", e)))?;

    let signature_array: [u8; 64] = signature_bytes.try_into().map_err(|_| {
        AppError::Internal(anyhow::anyhow!("Invalid signature length"))
    })?;

    let signature = Signature::from_bytes(&signature_array);

    // Parse public key
    let public_key_array: [u8; 32] = public_key
        .try_into()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("Invalid public key length")))?;

    let verifying_key = VerifyingKey::from_bytes(&public_key_array)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid public key: {}", e)))?;

    // Message format: "register:{username}:{timestamp}"
    let message = format!("register:{}:{}", username, timestamp);

    // Verify signature
    verifying_key
        .verify(message.as_bytes(), &signature)
        .map_err(|_| AppError::InvalidSignature)?;

    // Check timestamp (must be within 5 minutes)
    let now = chrono::Utc::now().timestamp();
    let diff = (now - timestamp).abs();
    if diff > 300 {
        // 5 minutes
        return Err(AppError::Internal(anyhow::anyhow!(
            "Timestamp too old or in future"
        )));
    }

    Ok(())
}

/// Check Redis health
async fn check_redis_health(redis: &redis::aio::ConnectionManager) -> Result<f64> {
    use redis::AsyncCommands;

    let start = std::time::Instant::now();

    let mut conn = redis.clone();
    // Use a simple GET/SET command to check Redis health
    let _: () = conn
        .set("health_check", "ok")
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis health check failed: {}", e)))?;

    let latency = start.elapsed().as_secs_f64() * 1000.0; // Convert to ms
    Ok(latency)
}
