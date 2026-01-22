//! Media Storage
//!
//! CRUD operations for media attachments (images, videos, files, voice messages).

use rusqlite::{params, Row};

use super::{Database, Result};

/// Media record
#[derive(Debug, Clone)]
pub struct Media {
    pub id: i64,
    pub media_hash: String,
    pub message_id: String,
    pub media_type: MediaType,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub local_path: Option<String>,
    pub thumbnail_path: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub created_at: i64,
}

/// New media to insert
#[derive(Debug, Clone)]
pub struct NewMedia {
    pub media_hash: String,
    pub message_id: String,
    pub media_type: MediaType,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub local_path: Option<String>,
    pub thumbnail_path: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
}

/// Media type enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
    VoiceMessage,
}

impl MediaType {
    pub fn as_str(&self) -> &str {
        match self {
            MediaType::Image => "image",
            MediaType::Video => "video",
            MediaType::Audio => "audio",
            MediaType::Document => "document",
            MediaType::VoiceMessage => "voice_message",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "image" => MediaType::Image,
            "video" => MediaType::Video,
            "audio" => MediaType::Audio,
            "document" => MediaType::Document,
            "voice_message" => MediaType::VoiceMessage,
            _ => MediaType::Document,
        }
    }

    /// Infer media type from MIME type
    pub fn from_mime_type(mime: &str) -> Self {
        if mime.starts_with("image/") {
            MediaType::Image
        } else if mime.starts_with("video/") {
            MediaType::Video
        } else if mime.starts_with("audio/") {
            MediaType::Audio
        } else {
            MediaType::Document
        }
    }
}

impl Database {
    /// Insert a new media attachment
    pub fn insert_media(&self, media: &NewMedia) -> Result<i64> {
        let conn = self.conn();
        conn.execute(
            r#"
            INSERT INTO media (
                media_hash, message_id, media_type, file_name, file_size,
                mime_type, local_path, thumbnail_path, width, height, duration_seconds
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            params![
                media.media_hash,
                media.message_id,
                media.media_type.as_str(),
                media.file_name,
                media.file_size,
                media.mime_type,
                media.local_path,
                media.thumbnail_path,
                media.width,
                media.height,
                media.duration_seconds,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get media by ID
    pub fn get_media(&self, media_id: i64) -> Result<Media> {
        let conn = self.conn();
        conn.query_row(
            r#"
            SELECT id, media_hash, message_id, media_type, file_name, file_size,
                   mime_type, local_path, thumbnail_path, width, height,
                   duration_seconds, created_at
            FROM media
            WHERE id = ?1
            "#,
            params![media_id],
            |row| self.media_from_row(row),
        )
        .map_err(Into::into)
    }

    /// Get media by hash (for deduplication)
    pub fn get_media_by_hash(&self, media_hash: &str) -> Result<Option<Media>> {
        let conn = self.conn();
        match conn.query_row(
            r#"
            SELECT id, media_hash, message_id, media_type, file_name, file_size,
                   mime_type, local_path, thumbnail_path, width, height,
                   duration_seconds, created_at
            FROM media
            WHERE media_hash = ?1
            "#,
            params![media_hash],
            |row| self.media_from_row(row),
        ) {
            Ok(media) => Ok(Some(media)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get all media for a message
    pub fn get_message_media(&self, message_id: &str) -> Result<Vec<Media>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            r#"
            SELECT id, media_hash, message_id, media_type, file_name, file_size,
                   mime_type, local_path, thumbnail_path, width, height,
                   duration_seconds, created_at
            FROM media
            WHERE message_id = ?1
            ORDER BY created_at
            "#,
        )?;

        let media = stmt
            .query_map(params![message_id], |row| self.media_from_row(row))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(media)
    }

    /// Get all media in a conversation (for gallery view)
    pub fn get_conversation_media(
        &self,
        conversation_id: &str,
        media_type: Option<MediaType>,
        limit: Option<usize>,
    ) -> Result<Vec<Media>> {
        let conn = self.conn();

        if let Some(media_type) = media_type {
            // Filter by media type
            let mut stmt = conn.prepare(
                r#"
                SELECT m.id, m.media_hash, m.message_id, m.media_type, m.file_name,
                       m.file_size, m.mime_type, m.local_path, m.thumbnail_path,
                       m.width, m.height, m.duration_seconds, m.created_at
                FROM media m
                JOIN messages msg ON m.message_id = msg.message_id
                WHERE msg.conversation_id = ?1 AND m.media_type = ?2
                ORDER BY m.created_at DESC
                LIMIT ?3
                "#,
            )?;

            let media = stmt
                .query_map(
                    params![conversation_id, media_type.as_str(), limit.unwrap_or(100)],
                    |row| self.media_from_row(row),
                )?
                .collect::<std::result::Result<Vec<_>, _>>()?;

            Ok(media)
        } else {
            // All media types
            let mut stmt = conn.prepare(
                r#"
                SELECT m.id, m.media_hash, m.message_id, m.media_type, m.file_name,
                       m.file_size, m.mime_type, m.local_path, m.thumbnail_path,
                       m.width, m.height, m.duration_seconds, m.created_at
                FROM media m
                JOIN messages msg ON m.message_id = msg.message_id
                WHERE msg.conversation_id = ?1
                ORDER BY m.created_at DESC
                LIMIT ?2
                "#,
            )?;

            let media = stmt
                .query_map(params![conversation_id, limit.unwrap_or(100)], |row| {
                    self.media_from_row(row)
                })?
                .collect::<std::result::Result<Vec<_>, _>>()?;

            Ok(media)
        }
    }

    /// Update media local path (after download)
    pub fn update_media_local_path(&self, media_id: i64, local_path: &str) -> Result<()> {
        self.conn().execute(
            "UPDATE media SET local_path = ?1 WHERE id = ?2",
            params![local_path, media_id],
        )?;
        Ok(())
    }

    /// Update media thumbnail path
    pub fn update_media_thumbnail_path(&self, media_id: i64, thumbnail_path: &str) -> Result<()> {
        self.conn().execute(
            "UPDATE media SET thumbnail_path = ?1 WHERE id = ?2",
            params![thumbnail_path, media_id],
        )?;
        Ok(())
    }

    /// Delete media (soft delete by removing local files)
    pub fn delete_media(&self, media_id: i64) -> Result<()> {
        self.conn().execute(
            "UPDATE media SET local_path = NULL, thumbnail_path = NULL WHERE id = ?1",
            params![media_id],
        )?;
        Ok(())
    }

    /// Helper: Parse media from row
    fn media_from_row(&self, row: &Row) -> rusqlite::Result<Media> {
        Ok(Media {
            id: row.get(0)?,
            media_hash: row.get(1)?,
            message_id: row.get(2)?,
            media_type: MediaType::from_str(&row.get::<_, String>(3)?),
            file_name: row.get(4)?,
            file_size: row.get(5)?,
            mime_type: row.get(6)?,
            local_path: row.get(7)?,
            thumbnail_path: row.get(8)?,
            width: row.get(9)?,
            height: row.get(10)?,
            duration_seconds: row.get(11)?,
            created_at: row.get(12)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{contacts::NewContact, migrations::migrate};

    fn setup_test_db() -> Database {
        let db = Database::in_memory().unwrap();
        migrate(&db).unwrap();

        // Insert test contacts
        let contact = NewContact {
            peer_id: "peer1".to_string(),
            username: None,
            display_name: Some("Peer 1".to_string()),
            public_key: vec![1, 2, 3],
            prekey_bundle_json: None,
        };
        db.insert_contact(&contact).unwrap();

        // Create conversation and message
        let conversation_id = db.get_or_create_conversation("peer1").unwrap();

        use crate::storage::messages::{MessageStatus, NewMessage};
        let message = NewMessage {
            message_id: "msg1".to_string(),
            conversation_id,
            sender_peer_id: "peer1".to_string(),
            recipient_peer_id: Some("peer2".to_string()),
            message_type: "text".to_string(),
            content_encrypted: None,
            content_plaintext: Some("Test message".to_string()),
            status: MessageStatus::Sent,
            parent_message_id: None,
        };
        db.insert_message(&message).unwrap();

        db
    }

    #[test]
    fn test_insert_and_get_media() {
        let db = setup_test_db();

        let new_media = NewMedia {
            media_hash: "hash123".to_string(),
            message_id: "msg1".to_string(),
            media_type: MediaType::Image,
            file_name: Some("photo.jpg".to_string()),
            file_size: Some(1024),
            mime_type: Some("image/jpeg".to_string()),
            local_path: Some("/path/to/photo.jpg".to_string()),
            thumbnail_path: Some("/path/to/thumb.jpg".to_string()),
            width: Some(640),
            height: Some(480),
            duration_seconds: None,
        };

        let media_id = db.insert_media(&new_media).unwrap();
        assert!(media_id > 0);

        let media = db.get_media(media_id).unwrap();
        assert_eq!(media.media_hash, "hash123");
        assert_eq!(media.media_type, MediaType::Image);
        assert_eq!(media.file_name, Some("photo.jpg".to_string()));
    }

    #[test]
    fn test_get_media_by_hash() {
        let db = setup_test_db();

        let new_media = NewMedia {
            media_hash: "unique_hash".to_string(),
            message_id: "msg1".to_string(),
            media_type: MediaType::Document,
            file_name: Some("doc.pdf".to_string()),
            file_size: Some(2048),
            mime_type: Some("application/pdf".to_string()),
            local_path: None,
            thumbnail_path: None,
            width: None,
            height: None,
            duration_seconds: None,
        };

        db.insert_media(&new_media).unwrap();

        let media = db.get_media_by_hash("unique_hash").unwrap();
        assert!(media.is_some());
        assert_eq!(media.unwrap().media_type, MediaType::Document);

        let not_found = db.get_media_by_hash("nonexistent").unwrap();
        assert!(not_found.is_none());
    }

    #[test]
    fn test_get_message_media() {
        let db = setup_test_db();

        // Insert 2 media for same message
        for i in 0..2 {
            let media = NewMedia {
                media_hash: format!("hash{}", i),
                message_id: "msg1".to_string(),
                media_type: MediaType::Image,
                file_name: Some(format!("photo{}.jpg", i)),
                file_size: Some(1024),
                mime_type: Some("image/jpeg".to_string()),
                local_path: None,
                thumbnail_path: None,
                width: Some(640),
                height: Some(480),
                duration_seconds: None,
            };
            db.insert_media(&media).unwrap();
        }

        let media_list = db.get_message_media("msg1").unwrap();
        assert_eq!(media_list.len(), 2);
    }

    #[test]
    fn test_media_type_from_mime() {
        assert_eq!(MediaType::from_mime_type("image/jpeg"), MediaType::Image);
        assert_eq!(MediaType::from_mime_type("video/mp4"), MediaType::Video);
        assert_eq!(MediaType::from_mime_type("audio/mpeg"), MediaType::Audio);
        assert_eq!(
            MediaType::from_mime_type("application/pdf"),
            MediaType::Document
        );
    }
}
