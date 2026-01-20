# MePassa Push Notification Server

Push notification service for the MePassa P2P messaging platform.

## Features

- **FCM Integration**: Send push notifications to Android devices via Firebase Cloud Messaging
- **Token Management**: Register, update, and deactivate device tokens
- **Multi-device Support**: Send notifications to all devices of a peer
- **Automatic Cleanup**: Mark invalid tokens as inactive
- **PostgreSQL Storage**: Store device tokens with metadata

## Architecture

- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with sqlx
- **Push Service**: Firebase Cloud Messaging (FCM)
- **Logging**: tracing with structured logs

## API Endpoints

### Health Check
```
GET /health
```

### Register Device Token
```
POST /api/v1/register
Content-Type: application/json

{
  "peer_id": "string",
  "platform": "fcm",  // or "apns" (future)
  "device_id": "string",
  "token": "string",
  "device_name": "string (optional)",
  "app_version": "string (optional)"
}
```

### Send Push Notification
```
POST /api/v1/send
Content-Type: application/json

{
  "peer_id": "string",
  "title": "string",
  "body": "string",
  "data": {  // optional
    "key": "value"
  }
}
```

### Unregister Device
```
DELETE /api/v1/unregister
Content-Type: application/json

{
  "peer_id": "string",
  "device_id": "string"
}
```

## Environment Variables

Create a `.env` file in the `server/push` directory:

```env
# Database
DATABASE_URL=postgresql://mepassa:mepassa_dev_password@localhost:5432/mepassa

# Firebase Cloud Messaging
FCM_SERVER_KEY=your_fcm_server_key_here

# Server (optional)
RUST_LOG=mepassa_push=debug,info
```

## Setup

### 1. Install Dependencies

```bash
# Rust toolchain (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL (via Docker)
cd /Users/edsonmartins/desenvolvimento/mepassa
docker-compose up -d postgres
```

### 2. Get FCM Server Key

1. Go to [Firebase Console](https://console.firebase.google.com)
2. Select your project (or create one)
3. Go to **Project Settings** → **Cloud Messaging**
4. Copy the **Server key** (legacy) or create a new one

### 3. Configure Environment

```bash
cd server/push
cp .env.example .env
# Edit .env and add your FCM_SERVER_KEY
```

### 4. Build

```bash
cargo build --release
```

Binary will be at: `../../target/release/mepassa-push`

## Running

### Development
```bash
cd server/push
cargo run
```

### Production
```bash
./target/release/mepassa-push
```

Server will start on `http://0.0.0.0:8081`

## Docker

### Build Image
```bash
docker build -t mepassa-push:latest .
```

### Run Container
```bash
docker run -d \
  --name mepassa-push \
  -p 8081:8081 \
  -e DATABASE_URL=postgresql://mepassa:password@postgres:5432/mepassa \
  -e FCM_SERVER_KEY=your_key \
  --network mepassa_network \
  mepassa-push:latest
```

## Testing

### Manual Test with curl

**1. Register a device:**
```bash
curl -X POST http://localhost:8081/api/v1/register \
  -H "Content-Type: application/json" \
  -d '{
    "peer_id": "test_peer_123",
    "platform": "fcm",
    "device_id": "device_001",
    "token": "fcm_token_here",
    "device_name": "Test Device",
    "app_version": "0.1.0"
  }'
```

**2. Send a notification:**
```bash
curl -X POST http://localhost:8081/api/v1/send \
  -H "Content-Type: application/json" \
  -d '{
    "peer_id": "test_peer_123",
    "title": "Test Notification",
    "body": "This is a test message",
    "data": {
      "type": "message",
      "from": "peer_456"
    }
  }'
```

**3. Unregister a device:**
```bash
curl -X DELETE http://localhost:8081/api/v1/unregister \
  -H "Content-Type: application/json" \
  -d '{
    "peer_id": "test_peer_123",
    "device_id": "device_001"
  }'
```

## Database Schema

The push server uses the `push_tokens` table:

```sql
CREATE TABLE push_tokens (
    id SERIAL PRIMARY KEY,
    peer_id VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL,  -- 'fcm' or 'apns'
    device_id VARCHAR(255) NOT NULL,
    token TEXT NOT NULL,
    device_name VARCHAR(255),
    app_version VARCHAR(50),
    created_at TIMESTAMP DEFAULT NOW(),
    last_used_at TIMESTAMP DEFAULT NOW(),
    is_active BOOLEAN DEFAULT true,
    UNIQUE(peer_id, device_id)
);
```

## Monitoring

### Logs
```bash
# Debug logs
RUST_LOG=mepassa_push=debug cargo run

# Info logs (default)
RUST_LOG=info cargo run
```

### Health Check
```bash
curl http://localhost:8081/health
# Response: OK
```

## Integration with MePassa Core

The push server is designed to integrate with the MePassa core system:

1. **On app startup**: Each client registers its FCM/APNs token
2. **On message send**: If peer is offline, trigger push notification
3. **On notification received**: Wake up app to poll Message Store

## Future Improvements

- [ ] APNs support for iOS (FASE 13)
- [ ] Rate limiting
- [ ] Notification analytics
- [ ] Retry logic for failed notifications
- [ ] Token expiration and cleanup
- [ ] Silent notifications (data-only)
- [ ] Rich notifications with images

## Troubleshooting

### "Database connection failed"
- Ensure PostgreSQL is running: `docker-compose up postgres`
- Check DATABASE_URL in .env
- Verify database exists: `psql -U mepassa -d mepassa -c "\dt"`

### "FCM send failed"
- Verify FCM_SERVER_KEY is correct
- Check device token is valid
- Ensure Firebase project has FCM enabled
- Check logs for specific FCM error codes

### "Token not found"
- Device must register before receiving notifications
- Check if token was marked inactive due to errors

## License

AGPL-3.0

## Authors

Integrall Tech <contato@integralltech.com.br>
