//! Firebase Cloud Messaging (FCM) client
//!
//! Handles sending push notifications to Android devices via FCM.

use std::collections::HashMap;

/// FCM Client for sending push notifications
pub struct FcmClient {
    client: fcm::Client,
    api_key: String,
}

impl FcmClient {
    /// Create a new FCM client with the given server key
    pub fn new(server_key: String) -> Self {
        Self {
            client: fcm::Client::new(),
            api_key: server_key,
        }
    }

    /// Send a push notification via FCM
    ///
    /// # Arguments
    /// * `token` - FCM device token
    /// * `title` - Notification title
    /// * `body` - Notification body
    /// * `data` - Additional custom data (optional)
    pub async fn send(
        &self,
        token: &str,
        title: &str,
        body: &str,
        data: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::debug!(
            "  🔥 Sending FCM notification - title: {}, body_len: {}",
            title,
            body.len()
        );

        // Build notification payload
        let mut notification_builder = fcm::NotificationBuilder::new();
        notification_builder.title(title);
        notification_builder.body(body);

        // Build message
        let mut message_builder = fcm::MessageBuilder::new(&self.api_key, token);
        message_builder.notification(notification_builder.finalize());

        // Add custom data
        if !data.is_empty() {
            let _ = message_builder.data(data);
        }

        // Send notification
        let response = self
            .client
            .send(message_builder.finalize())
            .await
            .map_err(|e| {
                tracing::error!("  ❌ FCM send error: {}", e);
                Box::new(e) as Box<dyn std::error::Error + Send + Sync>
            })?;

        // Check response
        if let Some(error) = response.error {
            tracing::error!("  ❌ FCM response error: {:?}", error);
            return Err(format!("FCM error: {:?}", error).into());
        }

        tracing::debug!("  ✅ FCM notification sent successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fcm_client_creation() {
        let client = FcmClient::new("test_server_key".to_string());
        assert!(!client.api_key.is_empty());
        assert_eq!(client.api_key, "test_server_key");
    }
}
