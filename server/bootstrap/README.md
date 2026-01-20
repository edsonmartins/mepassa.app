# MePassa Bootstrap Node

Bootstrap node for peer discovery in the MePassa P2P network using Kademlia DHT.

## Overview

The Bootstrap Node serves as the initial entry point for peers joining the MePassa network. It maintains a Kademlia DHT routing table to help peers discover each other and establish direct P2P connections.

## Features

- **Kademlia DHT**: Distributed hash table for peer discovery
- **Deterministic Peer ID**: Same peer ID generated on every restart using seed
- **Health Check**: HTTP endpoint for monitoring node status
- **Multiple Protocols**: Identify, Ping, AutoNAT for robust P2P connectivity
- **Structured Logging**: Detailed logs using tracing framework
- **Docker Support**: Easy deployment with docker-compose

## Configuration

Environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `BOOTSTRAP_PORT` | libp2p listening port | `4001` |
| `HEALTH_PORT` | HTTP health check port | `8000` |
| `PEER_ID_SEED` | Seed for deterministic peer ID | `bootstrap-1` |
| `DATA_DIR` | Directory for persistent storage | `/app/data` |
| `RUST_LOG` | Log level (trace, debug, info, warn, error) | `info` |

## Running

### Docker (recommended)

Start the bootstrap node with docker-compose:

```bash
docker-compose up bootstrap-node-1
```

View logs:

```bash
docker logs -f mepassa-bootstrap-1
```

### Local Development

```bash
cd server/bootstrap
cargo run
```

With custom configuration:

```bash
BOOTSTRAP_PORT=4001 RUST_LOG=debug cargo run
```

## Health Check

The bootstrap node exposes a health check endpoint on port 8000 (configurable via `HEALTH_PORT`).

```bash
curl http://localhost:8000/health
```

Example response:

```json
{
  "status": "OK",
  "peer_count": 42,
  "uptime_seconds": 3600
}
```

## Client Connection

To connect to the bootstrap node from a MePassa client:

```rust
use libp2p::{Multiaddr, PeerId};

// Bootstrap node details (get peer_id from bootstrap logs)
let bootstrap_peer_id: PeerId = "12D3KooW...".parse()?;
let bootstrap_addr: Multiaddr = "/ip4/127.0.0.1/tcp/4001".parse()?;

// Connect to bootstrap
client.connect_to_peer(bootstrap_peer_id, bootstrap_addr).await?;

// Start DHT bootstrap process
client.bootstrap().await?;
```

The bootstrap node peer ID will be printed in the logs when it starts:

```
🚀 MePassa Bootstrap Node starting...
   Peer ID: 12D3KooWABC123...
   Listening on: /ip4/0.0.0.0/tcp/4001
✅ Bootstrap node ready!
```

## Architecture

### Protocols

- **Kademlia DHT**: Peer discovery and routing
- **Identify**: Exchange peer information (agent, protocols, addresses)
- **Ping**: Keep connections alive and measure latency
- **AutoNAT**: Detect NAT type and reachability

### Transport Stack

```
Application Layer
    ↓
Yamux (multiplexing)
    ↓
Noise (encryption)
    ↓
TCP (transport)
```

### Event Loop

The bootstrap node runs an event loop that handles:

1. Incoming connections from peers
2. DHT queries (FindNode, GetProviders, GetRecord)
3. Identify requests (peer info exchange)
4. Connection lifecycle events
5. Health metrics updates

## Networking

### Ports

- **4001/tcp**: P2P connections (libp2p)
- **8000/tcp**: Health check HTTP API

### Firewall Rules

Ensure port 4001 is accessible from the internet for peers to connect:

```bash
# Example: UFW
sudo ufw allow 4001/tcp

# Example: iptables
sudo iptables -A INPUT -p tcp --dport 4001 -j ACCEPT
```

## Monitoring

### Logs

Structured logs using the tracing framework. Log levels:

- `error`: Critical errors
- `warn`: Warnings (connection failures, etc.)
- `info`: Important events (new connections, DHT updates)
- `debug`: Detailed protocol events
- `trace`: Very verbose debugging

Change log level:

```bash
RUST_LOG=debug docker-compose up bootstrap-node-1
```

### Metrics

Health endpoint provides:

- `status`: "OK" if running
- `peer_count`: Number of currently connected peers
- `uptime_seconds`: Time since node started

## Troubleshooting

### Port already in use

If port 4001 is already in use:

```bash
# Change the port
BOOTSTRAP_PORT=4002 cargo run
```

Or modify `docker-compose.yml`:

```yaml
environment:
  - BOOTSTRAP_PORT=4002
ports:
  - "4002:4002"
```

### No peers connecting

1. Check firewall rules allow port 4001
2. Verify bootstrap node is running: `curl http://localhost:8000/health`
3. Check logs for errors: `docker logs mepassa-bootstrap-1`
4. Ensure clients are using correct peer ID and multiaddr

### High CPU usage

This is normal during DHT queries. If sustained:

1. Check number of connected peers (health endpoint)
2. Review logs for repeated errors
3. Consider rate limiting (future feature)

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
# Run unit tests
cargo test

# Run with verbose logging
RUST_LOG=trace cargo run
```

### Code Structure

```
src/
├── main.rs         - Entry point, event loop
├── config.rs       - Configuration from env vars
├── behaviour.rs    - libp2p NetworkBehaviour
├── health.rs       - HTTP health check server
└── storage.rs      - Persistent storage (future)
```

## Future Improvements

- [ ] Persistent DHT storage (survive restarts)
- [ ] Prometheus metrics endpoint
- [ ] Web dashboard for monitoring
- [ ] Rate limiting for DHT queries
- [ ] Multiple bootstrap nodes with failover
- [ ] Geographic routing optimization

## License

AGPL-3.0

## Support

For issues or questions, open an issue on GitHub or contact: contato@integralltech.com.br
