# MePassa Makefile
# Quick commands for development

.PHONY: help setup up down logs clean build test fmt lint dmg

help: ## Show this help
	@echo "MePassa Development Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Initial setup (copy .env, install deps)
	@echo "Setting up MePassa development environment..."
	@if [ ! -f .env ]; then cp .env.example .env && echo "Created .env file"; fi
	@echo "Installing Rust dependencies..."
	@cd core && cargo fetch
	@echo "Setup complete!"

up: ## Start all services
	docker-compose up -d
	@echo "Services started. Use 'make logs' to view logs."

up-monitoring: ## Start all services including monitoring stack
	docker-compose --profile monitoring up -d
	@echo "Services started with monitoring."
	@echo "Grafana: http://localhost:3000 (admin/admin)"
	@echo "Prometheus: http://localhost:9090"

down: ## Stop all services
	docker-compose down

down-volumes: ## Stop all services and remove volumes
	docker-compose down -v
	@echo "All volumes removed!"

logs: ## Show logs from all services
	docker-compose logs -f

logs-postgres: ## Show PostgreSQL logs
	docker-compose logs -f postgres

logs-redis: ## Show Redis logs
	docker-compose logs -f redis

logs-coturn: ## Show TURN server logs
	docker-compose logs -f coturn

logs-bootstrap: ## Show bootstrap node logs
	docker-compose logs -f bootstrap-node-1

logs-store: ## Show message store logs
	docker-compose logs -f message-store

logs-push: ## Show push server logs
	docker-compose logs -f push-server

clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	@cd core && cargo clean
	@rm -rf target/
	@echo "Clean complete!"

build: ## Build Rust workspace
	@echo "Building Rust workspace..."
	@cargo build --workspace
	@echo "Build complete!"

build-release: ## Build Rust workspace (release mode)
	@echo "Building Rust workspace (release)..."
	@cargo build --workspace --release
	@echo "Build complete!"

test: ## Run all tests
	@echo "Running tests..."
	@cargo test --workspace
	@echo "Tests complete!"

fmt: ## Format code
	@echo "Formatting code..."
	@cargo fmt --all
	@echo "Format complete!"

dmg: ## Build DMG do desktop (macOS)
	@echo "Building macOS DMG..."
	@./scripts/build-dmg.sh
	@echo "DMG build complete!"

lint: ## Run clippy
	@echo "Running clippy..."
	@cargo clippy --workspace --all-features -- -D warnings
	@echo "Lint complete!"

check: fmt lint test ## Run format, lint, and test

db-shell: ## Open PostgreSQL shell
	docker-compose exec postgres psql -U mepassa -d mepassa

redis-cli: ## Open Redis CLI
	docker-compose exec redis redis-cli -a mepassa_redis_dev

db-reset: ## Reset database (WARNING: deletes all data)
	@echo "WARNING: This will delete all database data!"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		docker-compose down -v postgres; \
		docker-compose up -d postgres; \
		echo "Database reset complete!"; \
	fi

restart: down up ## Restart all services

status: ## Show service status
	docker-compose ps

dev-core: ## Run core development server
	@cd core && cargo watch -x 'run --example simple_chat'

dev-android: ## Run Android app
	@cd android && ./gradlew installDebug && adb shell am start -n com.mepassa/.MainActivity

dev-desktop: ## Run Desktop app
	@cd desktop && npm run tauri dev

# Health checks
health: ## Check service health
	@echo "Checking service health..."
	@echo -n "PostgreSQL: "
	@docker-compose exec -T postgres pg_isready -U mepassa > /dev/null 2>&1 && echo "✓ OK" || echo "✗ FAILED"
	@echo -n "Redis: "
	@docker-compose exec -T redis redis-cli -a mepassa_redis_dev ping > /dev/null 2>&1 && echo "✓ OK" || echo "✗ FAILED"
	@echo -n "Message Store: "
	@curl -sf http://localhost:8080/health > /dev/null 2>&1 && echo "✓ OK" || echo "✗ FAILED"
	@echo -n "Push Server: "
	@curl -sf http://localhost:8081/health > /dev/null 2>&1 && echo "✓ OK" || echo "✗ FAILED"

# Documentation
docs: ## Generate and open documentation
	@cargo doc --workspace --no-deps --open

# Install tools
install-tools: ## Install development tools
	@echo "Installing development tools..."
	@cargo install cargo-watch
	@cargo install cargo-edit
	@cargo install uniffi-bindgen
	@echo "Tools installed!"
