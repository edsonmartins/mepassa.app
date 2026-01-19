//! Identity Server library

pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod rate_limit;

use redis::aio::ConnectionManager;
use sqlx::PgPool;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub start_time: std::time::Instant,
}

impl AppState {
    pub fn new(db: PgPool, redis: ConnectionManager) -> Self {
        Self {
            db,
            redis,
            start_time: std::time::Instant::now(),
        }
    }
}
