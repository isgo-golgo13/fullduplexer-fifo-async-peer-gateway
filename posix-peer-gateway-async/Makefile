# Variables
APP_NAME := full_duplexer_fifo_async_peer_gateway
PEER_BINARY := target/release/async-posix-peer
GATEWAY_BINARY := target/release/async-posix-peer-gateway
DOCKER_COMPOSE_FILE := docker-compose.yml

# Default target to build the peer and gateway binaries
.PHONY: all
all: build-peer build-gateway

# Build the peer binary
.PHONY: build-peer
build-peer:
	cargo build --release --bin async-posix-peer
	@echo "Peer binary built: $(PEER_BINARY)"

# Build the gateway binary
.PHONY: build-gateway
build-gateway:
	cargo build --release --bin async-posix-peer-gateway
	@echo "Gateway binary built: $(GATEWAY_BINARY)"

# Run the peer binary directly
.PHONY: run-peer
run-peer: build-peer
	./$(PEER_BINARY)

# Run the gateway binary directly
.PHONY: run-gateway
run-gateway: build-gateway
	./$(GATEWAY_BINARY)

# Build the Docker image for peer
.PHONY: build-peer-image
build-peer-image:
	docker build -f Dockerfile.peer -t $(APP_NAME)_peer .

# Build the Docker image for gateway
.PHONY: build-gateway-image
build-gateway-image:
	docker build -f Dockerfile.gateway -t $(APP_NAME)_gateway .

# Build both Docker images
.PHONY: build-images
build-images: build-peer-image build-gateway-image

# Run Docker Compose up
.PHONY: up
up:
	docker-compose -f $(DOCKER_COMPOSE_FILE) up -d

# Run Docker Compose down
.PHONY: down
down:
	docker-compose -f $(DOCKER_COMPOSE_FILE) down

# Clean the project by removing the target directory
.PHONY: clean
clean:
	cargo clean
	@echo "Project cleaned: target directory removed."
