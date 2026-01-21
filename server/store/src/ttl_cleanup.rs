//! Background job for TTL cleanup

use std::time::Duration;
use tokio::time;

use crate::database::Database;

/// TTL cleanup job configuration
pub struct TtlCleanupJob {
    db: Database,
    interval: Duration,
}

impl TtlCleanupJob {
    /// Create a new TTL cleanup job
    pub fn new(db: Database) -> Self {
        Self {
            db,
            interval: Duration::from_secs(3600), // Run every hour
        }
    }

    /// Create with custom interval
    pub fn with_interval(db: Database, interval: Duration) -> Self {
        Self { db, interval }
    }

    /// Start the cleanup job
    pub async fn start(self) {
        tracing::info!(
            "🧹 TTL cleanup job started (interval: {} seconds)",
            self.interval.as_secs()
        );

        let mut interval_timer = time::interval(self.interval);

        loop {
            interval_timer.tick().await;

            tracing::debug!("🧹 Running TTL cleanup...");

            match self.db.delete_expired_messages().await {
                Ok(deleted) => {
                    if deleted > 0 {
                        tracing::info!("🗑️ TTL cleanup: deleted {} expired messages", deleted);
                    } else {
                        tracing::debug!("🧹 TTL cleanup: no expired messages");
                    }
                }
                Err(e) => {
                    tracing::error!("❌ TTL cleanup failed: {:?}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ttl_job_creation() {
        // Test is mainly for compilation
        // Actual testing requires database
    }
}
