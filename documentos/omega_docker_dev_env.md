# Omega Chat - Docker Development Environment

## docker-compose.yml

```yaml
version: '3.8'

services:
  # ========================================
  # Bootstrap/Discovery Nodes (DHT)
  # ========================================
  bootstrap-1:
    build:
      context: ./server/discovery
      dockerfile: Dockerfile
    container_name: omega-bootstrap-1
    environment:
      - RUST_LOG=info
      - BOOTSTRAP_MODE=true
      - LISTEN_ADDR=/ip4/0.0.0.0/tcp/4001
    ports:
      - "4001:4001"
    networks:
      omega-net:
        ipv4_address: 172.20.0.10
    restart: unless-stopped

  bootstrap-2:
    build:
      context: ./server/discovery
      dockerfile: Dockerfile
    container_name: omega-bootstrap-2
    environment:
      - RUST_LOG=info
      - BOOTSTRAP_MODE=true
      - LISTEN_ADDR=/ip4/0.0.0.0/tcp/4002
      - PEER_BOOTSTRAP=/ip4/172.20.0.10/tcp/4001
    ports:
      - "4002:4002"
    networks:
      omega-net:
        ipv4_address: 172.20.0.11
    depends_on:
      - bootstrap-1
    restart: unless-stopped

  # ========================================
  # Message Store (Redis + PostgreSQL)
  # ========================================
  redis:
    image: redis:7-alpine
    container_name: omega-redis
    command: redis-server --appendonly yes --maxmemory 512mb --maxmemory-policy allkeys-lru
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data
    networks:
      - omega-net
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 3s
      retries: 3

  postgres:
    image: postgres:16-alpine
    container_name: omega-postgres
    environment:
      POSTGRES_DB: omega_store
      POSTGRES_USER: omega
      POSTGRES_PASSWORD: omega_dev_password
      PGDATA: /var/lib/postgresql/data/pgdata
    ports:
      - "5432:5432"
    volumes:
      - postgres-data:/var/lib/postgresql/data
      - ./server/store/init.sql:/docker-entrypoint-initdb.d/init.sql
    networks:
      - omega-net
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U omega"]
      interval: 10s
      timeout: 3s
      retries: 3

  message-store:
    build:
      context: ./server/store
      dockerfile: Dockerfile
    container_name: omega-message-store
    environment:
      - RUST_LOG=info
      - REDIS_URL=redis://redis:6379
      - DATABASE_URL=postgresql://omega:omega_dev_password@postgres:5432/omega_store
      - TTL_DAYS=14
      - GRPC_PORT=50051
    ports:
      - "50051:50051"
      - "8081:8081" # HTTP metrics
    networks:
      - omega-net
    depends_on:
      redis:
        condition: service_healthy
      postgres:
        condition: service_healthy
    restart: unless-stopped

  # ========================================
  # Relay Server (TURN/Relay)
  # ========================================
  coturn:
    image: coturn/coturn:latest
    container_name: omega-coturn
    command: |
      -n
      --listening-ip=0.0.0.0
      --relay-ip=172.20.0.20
      --external-ip=$${EXTERNAL_IP:-127.0.0.1}
      --min-port=49152
      --max-port=65535
      --realm=omega.chat
      --user=omega:omega_turn_password
      --log-file=stdout
      --simple-log
    ports:
      - "3478:3478/tcp"
      - "3478:3478/udp"
      - "5349:5349/tcp"
      - "5349:5349/udp"
      - "49152-49252:49152-49252/udp" # Relay port range (limited for dev)
    networks:
      omega-net:
        ipv4_address: 172.20.0.20
    restart: unless-stopped

  # ========================================
  # Monitoring
  # ========================================
  prometheus:
    image: prom/prometheus:latest
    container_name: omega-prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/usr/share/prometheus/console_libraries'
      - '--web.console.templates=/usr/share/prometheus/consoles'
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    networks:
      - omega-net
    restart: unless-stopped

  grafana:
    image: grafana/grafana:latest
    container_name: omega-grafana
    environment:
      - GF_SECURITY_ADMIN_USER=admin
      - GF_SECURITY_ADMIN_PASSWORD=omega_admin
      - GF_USERS_ALLOW_SIGN_UP=false
    ports:
      - "3000:3000"
    volumes:
      - grafana-data:/var/lib/grafana
      - ./monitoring/grafana/dashboards:/etc/grafana/provisioning/dashboards
      - ./monitoring/grafana/datasources:/etc/grafana/provisioning/datasources
    networks:
      - omega-net
    depends_on:
      - prometheus
    restart: unless-stopped

  # ========================================
  # Desenvolvimento
  # ========================================
  rust-dev:
    build:
      context: .
      dockerfile: Dockerfile.dev
    container_name: omega-rust-dev
    volumes:
      - .:/workspace
      - cargo-cache:/usr/local/cargo/registry
      - target-cache:/workspace/target
    working_dir: /workspace
    command: sleep infinity
    networks:
      - omega-net
    environment:
      - RUST_LOG=debug
      - BOOTSTRAP_PEERS=/ip4/172.20.0.10/tcp/4001,/ip4/172.20.0.11/tcp/4002

networks:
  omega-net:
    driver: bridge
    ipam:
      config:
        - subnet: 172.20.0.0/16

volumes:
  redis-data:
  postgres-data:
  prometheus-data:
  grafana-data:
  cargo-cache:
  target-cache:
```

## Dockerfile.dev (Desenvolvimento Rust)

```dockerfile
# Dockerfile.dev
FROM rust:1.75-bullseye

# Instalar dependências do sistema
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    protobuf-compiler \
    clang \
    llvm \
    git \
    curl \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Instalar ferramentas Rust
RUN rustup component add rustfmt clippy
RUN cargo install cargo-watch cargo-edit cargo-audit cargo-deny

# Instalar uniffi-bindgen
RUN cargo install uniffi-bindgen-go uniffi-bindgen-kotlin uniffi-bindgen-swift

WORKDIR /workspace

# Cache de dependências
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

CMD ["sleep", "infinity"]
```

## server/discovery/Dockerfile

```dockerfile
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --bin omega-discovery

FROM alpine:latest
RUN apk add --no-cache ca-certificates

COPY --from=builder /build/target/release/omega-discovery /usr/local/bin/

EXPOSE 4001

ENTRYPOINT ["omega-discovery"]
```

## server/store/Dockerfile

```dockerfile
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev postgresql-dev

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --bin omega-store

FROM alpine:latest
RUN apk add --no-cache ca-certificates libpq

COPY --from=builder /build/target/release/omega-store /usr/local/bin/

EXPOSE 50051 8081

ENTRYPOINT ["omega-store"]
```

## server/store/init.sql

```sql
-- init.sql

CREATE TABLE IF NOT EXISTS messages (
    message_id UUID PRIMARY KEY,
    recipient_hash VARCHAR(64) NOT NULL,
    encrypted_payload BYTEA NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ttl_seconds INTEGER DEFAULT 1209600, -- 14 dias
    delivered BOOLEAN DEFAULT FALSE,
    delivered_at TIMESTAMP WITH TIME ZONE,
    
    -- Metadados (não sensíveis)
    payload_size INTEGER,
    sender_hash VARCHAR(64), -- Hash do sender (não identidade real)
    
    -- Índices
    CONSTRAINT ttl_positive CHECK (ttl_seconds > 0)
);

-- Índices para performance
CREATE INDEX idx_recipient_ttl ON messages(recipient_hash, created_at) 
    WHERE delivered = FALSE;

CREATE INDEX idx_created_ttl ON messages(created_at) 
    WHERE delivered = FALSE;

-- Função para auto-deletar mensagens expiradas
CREATE OR REPLACE FUNCTION delete_expired_messages()
RETURNS void AS $$
BEGIN
    DELETE FROM messages 
    WHERE created_at + (ttl_seconds * INTERVAL '1 second') < NOW()
       OR (delivered = TRUE AND delivered_at + INTERVAL '1 hour' < NOW());
END;
$$ LANGUAGE plpgsql;

-- Tabela de métricas (opcional)
CREATE TABLE IF NOT EXISTS metrics (
    id SERIAL PRIMARY KEY,
    metric_name VARCHAR(100) NOT NULL,
    metric_value BIGINT NOT NULL,
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_metrics_name_time ON metrics(metric_name, recorded_at DESC);

-- Estatísticas iniciais
INSERT INTO metrics (metric_name, metric_value) VALUES 
    ('messages_stored', 0),
    ('messages_delivered', 0),
    ('messages_expired', 0);
```

## monitoring/prometheus.yml

```yaml
# prometheus.yml

global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'omega-message-store'
    static_configs:
      - targets: ['message-store:8081']
        labels:
          service: 'message-store'
  
  - job_name: 'omega-bootstrap'
    static_configs:
      - targets: ['bootstrap-1:9091', 'bootstrap-2:9091']
        labels:
          service: 'bootstrap'
  
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres:5432']
        labels:
          service: 'postgres'
```

## monitoring/grafana/datasources/datasource.yml

```yaml
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
    editable: true
```

## scripts/dev-setup.sh

```bash
#!/bin/bash
# scripts/dev-setup.sh

set -e

echo "🚀 Setting up Omega development environment..."

# Verificar Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker não encontrado. Instale Docker primeiro."
    exit 1
fi

# Verificar Docker Compose
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose não encontrado. Instale Docker Compose primeiro."
    exit 1
fi

# Criar diretórios necessários
mkdir -p monitoring/grafana/{dashboards,datasources}
mkdir -p server/{discovery,store,relay}
mkdir -p logs

# Detectar IP externo para TURN server
EXTERNAL_IP=$(curl -s ifconfig.me || echo "127.0.0.1")
export EXTERNAL_IP

echo "📡 External IP detectado: $EXTERNAL_IP"

# Build e start dos containers
echo "🐳 Building containers..."
docker-compose build

echo "🚀 Starting services..."
docker-compose up -d

echo "⏳ Aguardando services ficarem healthy..."
sleep 10

# Verificar health
docker-compose ps

echo "
✅ Ambiente de desenvolvimento configurado!

📊 Serviços disponíveis:
  - Bootstrap Node 1: localhost:4001
  - Bootstrap Node 2: localhost:4002
  - Message Store gRPC: localhost:50051
  - Message Store Metrics: localhost:8081
  - TURN Server: localhost:3478 (TCP/UDP)
  - PostgreSQL: localhost:5432 (user: omega, pass: omega_dev_password)
  - Redis: localhost:6379
  - Prometheus: http://localhost:9090
  - Grafana: http://localhost:3000 (admin/omega_admin)

🛠️  Para desenvolvimento:
  docker-compose exec rust-dev bash

📝 Logs:
  docker-compose logs -f [service_name]

🛑 Para parar:
  docker-compose down

🗑️  Para limpar tudo (incluindo volumes):
  docker-compose down -v
"
```

## scripts/test-network.sh

```bash
#!/bin/bash
# scripts/test-network.sh

set -e

echo "🧪 Testando conectividade da rede Omega..."

# Test Bootstrap Nodes
echo "📡 Testando Bootstrap Nodes..."
for port in 4001 4002; do
    if nc -zv localhost $port 2>&1 | grep -q succeeded; then
        echo "  ✅ Bootstrap node em porta $port está UP"
    else
        echo "  ❌ Bootstrap node em porta $port está DOWN"
    fi
done

# Test Message Store
echo "📦 Testando Message Store..."
if nc -zv localhost 50051 2>&1 | grep -q succeeded; then
    echo "  ✅ Message Store gRPC está UP"
else
    echo "  ❌ Message Store gRPC está DOWN"
fi

# Test TURN Server
echo "🔄 Testando TURN Server..."
if nc -zuv localhost 3478 2>&1 | grep -q succeeded; then
    echo "  ✅ TURN Server está UP"
else
    echo "  ❌ TURN Server está DOWN"
fi

# Test PostgreSQL
echo "🗄️  Testando PostgreSQL..."
if PGPASSWORD=omega_dev_password psql -h localhost -U omega -d omega_store -c "SELECT 1" &> /dev/null; then
    echo "  ✅ PostgreSQL está UP e acessível"
    
    # Count messages
    MESSAGE_COUNT=$(PGPASSWORD=omega_dev_password psql -h localhost -U omega -d omega_store -t -c "SELECT COUNT(*) FROM messages" | xargs)
    echo "  📊 Mensagens armazenadas: $MESSAGE_COUNT"
else
    echo "  ❌ PostgreSQL connection failed"
fi

# Test Redis
echo "💾 Testando Redis..."
if redis-cli -h localhost ping | grep -q PONG; then
    echo "  ✅ Redis está UP"
    
    # Info
    REDIS_KEYS=$(redis-cli -h localhost DBSIZE | grep -oE '[0-9]+')
    echo "  📊 Keys no Redis: $REDIS_KEYS"
else
    echo "  ❌ Redis connection failed"
fi

echo "
✅ Testes de rede concluídos!
"
```

## scripts/generate-test-messages.sh

```bash
#!/bin/bash
# scripts/generate-test-messages.sh

set -e

echo "📨 Gerando mensagens de teste..."

# Inserir mensagens de teste no PostgreSQL
for i in {1..10}; do
    UUID=$(uuidgen)
    RECIPIENT_HASH=$(echo -n "recipient_$i" | sha256sum | cut -d' ' -f1 | cut -c1-64)
    PAYLOAD=$(echo "Test message $i" | base64)
    
    PGPASSWORD=omega_dev_password psql -h localhost -U omega -d omega_store -c "
        INSERT INTO messages (message_id, recipient_hash, encrypted_payload, payload_size)
        VALUES ('$UUID', '$RECIPIENT_HASH', decode('$PAYLOAD', 'base64'), length('$PAYLOAD'))
    "
    
    echo "  ✅ Mensagem $i inserida (recipient_hash: ${RECIPIENT_HASH:0:16}...)"
done

# Estatísticas
TOTAL=$(PGPASSWORD=omega_dev_password psql -h localhost -U omega -d omega_store -t -c "SELECT COUNT(*) FROM messages" | xargs)
UNDELIVERED=$(PGPASSWORD=omega_dev_password psql -h localhost -U omega -d omega_store -t -c "SELECT COUNT(*) FROM messages WHERE delivered = FALSE" | xargs)

echo "
📊 Estatísticas:
  Total de mensagens: $TOTAL
  Não entregues: $UNDELIVERED
"
```

## Makefile

```makefile
# Makefile

.PHONY: help setup up down logs test clean build-core

help:
	@echo "Omega Chat - Development Commands"
	@echo ""
	@echo "  make setup      - Setup development environment"
	@echo "  make up         - Start all services"
	@echo "  make down       - Stop all services"
	@echo "  make logs       - Show logs (all services)"
	@echo "  make test       - Run tests"
	@echo "  make clean      - Clean everything (including volumes)"
	@echo "  make build-core - Build omega-core library"
	@echo "  make shell      - Enter development container"

setup:
	@bash scripts/dev-setup.sh

up:
	@docker-compose up -d
	@echo "Services started. Check status with: make logs"

down:
	@docker-compose down

logs:
	@docker-compose logs -f

test:
	@bash scripts/test-network.sh

clean:
	@docker-compose down -v
	@echo "All containers and volumes removed"

build-core:
	@docker-compose exec rust-dev cargo build --release
	@echo "omega-core built successfully"

shell:
	@docker-compose exec rust-dev bash

test-messages:
	@bash scripts/generate-test-messages.sh

clippy:
	@docker-compose exec rust-dev cargo clippy --all-targets --all-features

fmt:
	@docker-compose exec rust-dev cargo fmt --all

watch:
	@docker-compose exec rust-dev cargo watch -x 'build' -x 'test'
```

## README para desenvolvimento

```markdown
# Omega Chat - Development Environment

## Quick Start

```bash
# 1. Clone o repositório
git clone https://github.com/integralltech/omega
cd omega

# 2. Setup ambiente
make setup

# 3. Verificar se tudo está funcionando
make test

# 4. Acessar container de desenvolvimento
make shell

# Dentro do container:
cd core
cargo build
cargo test
```

## Arquitetura do ambiente

```
┌─────────────────────────────────────────────┐
│         Host Machine (Linux/macOS)           │
├─────────────────────────────────────────────┤
│                                              │
│  Docker Containers:                          │
│  ├── bootstrap-1 (DHT node)                  │
│  ├── bootstrap-2 (DHT node)                  │
│  ├── message-store (gRPC server)             │
│  ├── postgres (database)                     │
│  ├── redis (cache)                           │
│  ├── coturn (TURN/STUN)                      │
│  ├── prometheus (metrics)                    │
│  ├── grafana (dashboards)                    │
│  └── rust-dev (development)                  │
│                                              │
└─────────────────────────────────────────────┘
```

## Comandos úteis

```bash
# Ver logs de um serviço específico
docker-compose logs -f message-store

# Resetar banco de dados
docker-compose exec postgres psql -U omega -d omega_store -c "TRUNCATE messages;"

# Gerar mensagens de teste
make test-messages

# Build e test
docker-compose exec rust-dev cargo test

# Watch mode (rebuild on changes)
make watch
```

## Portas expostas

| Serviço | Porta | Descrição |
|---------|-------|-----------|
| Bootstrap 1 | 4001 | DHT node |
| Bootstrap 2 | 4002 | DHT node |
| Message Store | 50051 | gRPC API |
| Message Store | 8081 | Metrics |
| PostgreSQL | 5432 | Database |
| Redis | 6379 | Cache |
| TURN | 3478 | STUN/TURN |
| Prometheus | 9090 | Metrics |
| Grafana | 3000 | Dashboards |
```

---

Este ambiente Docker fornece tudo que você precisa para desenvolver e testar o Omega Chat localmente, sem precisar configurar nada manualmente!
