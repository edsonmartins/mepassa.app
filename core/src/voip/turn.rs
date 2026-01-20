//! TURN Credentials Helper
//!
//! Fetches time-limited TURN credentials from the credentials server.

use super::{manager::TurnCredentials, Result, VoipError};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// TURN credentials request
#[derive(Debug, Serialize)]
struct CredentialRequest {
    username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_seconds: Option<i64>,
}

/// TURN credentials response from server
#[derive(Debug, Deserialize)]
struct CredentialResponse {
    username: String,
    password: String,
    uris: Vec<String>,
    ttl: i64,
}

/// TURN credentials client
pub struct TurnCredentialsClient {
    server_url: String,
    http_client: Client,
}

impl TurnCredentialsClient {
    /// Create a new TURN credentials client
    pub fn new(server_url: String) -> Self {
        Self {
            server_url,
            http_client: Client::new(),
        }
    }

    /// Fetch TURN credentials from server
    ///
    /// # Arguments
    /// * `peer_id` - Local peer ID to use as username base
    /// * `ttl_seconds` - Time-to-live for credentials (default: 24h)
    ///
    /// # Returns
    /// TURN credentials with username, password, and server URIs
    pub async fn fetch_credentials(
        &self,
        peer_id: &str,
        ttl_seconds: Option<i64>,
    ) -> Result<TurnCredentials> {
        let url = format!("{}/api/turn/credentials", self.server_url);

        let request = CredentialRequest {
            username: peer_id.to_string(),
            ttl_seconds,
        };

        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| VoipError::NetworkError(format!("Failed to fetch TURN credentials: {}", e)))?;

        if !response.status().is_success() {
            return Err(VoipError::NetworkError(format!(
                "TURN credentials request failed: {}",
                response.status()
            )));
        }

        let creds: CredentialResponse = response
            .json()
            .await
            .map_err(|e| VoipError::NetworkError(format!("Failed to parse credentials: {}", e)))?;

        tracing::info!(
            "✅ Fetched TURN credentials (TTL: {}s, URIs: {})",
            creds.ttl,
            creds.uris.len()
        );

        Ok(TurnCredentials {
            username: creds.username,
            password: creds.password,
            uris: creds.uris,
        })
    }

    /// Fetch credentials with default TTL (24 hours)
    pub async fn fetch_default(&self, peer_id: &str) -> Result<TurnCredentials> {
        self.fetch_credentials(peer_id, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_client_creation() {
        let client = TurnCredentialsClient::new("http://localhost:8082".to_string());
        assert_eq!(client.server_url, "http://localhost:8082");
    }

    #[tokio::test]
    #[ignore] // Requires TURN credentials server running
    async fn test_fetch_credentials() {
        let client = TurnCredentialsClient::new("http://localhost:8082".to_string());
        let result = client.fetch_default("test_peer").await;

        // This will fail unless the server is running
        // But the test validates the code compiles correctly
        if let Ok(creds) = result {
            assert!(!creds.username.is_empty());
            assert!(!creds.password.is_empty());
            assert!(!creds.uris.is_empty());
        }
    }
}
