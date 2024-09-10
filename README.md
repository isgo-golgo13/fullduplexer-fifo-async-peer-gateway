# fullduplexer-fifo-async-peer-gateway
Rust FullDuplex POSIX FIFO Async Peers and Central FIFO Peer Gateway using POSIX Libc

## Project Source File Structure
The following is the project source file structure.

```shell
posix-peer-gateway-async/
├── Dockerfile.gateway
├── Dockerfile.peer
├── docker-compose.yml
├── Cargo.toml
├── Cargo.lock
├── .env
├── src/
│   └── main.rs
├── svckit/
│   ├── lib.rs
│   ├── fullduplexer_async.rs
│   ├── fullduplex-async-peer.rs
│   └── fullduplex-async-peer-gateway.rs
```




## Build Docker Images and Run

The provided `docker-compose.yaml`file is as follows.

```yaml
version: "3.8"
services:
  gateway:
    build:
      context: .
      dockerfile: Dockerfile.gateway # Using the async gateway Dockerfile
    container_name: async_posix_gateway
    restart: unless-stopped
    environment:
      - RUST_LOG=info # Optional: Adjust logging levels
    networks:
      - posix_network

  peer1:
    build:
      context: .
      dockerfile: Dockerfile.peer # Using the async peer Dockerfile
    container_name: async_posix_peer1
    restart: unless-stopped
    depends_on:
      - gateway
    environment:
      - RUST_LOG=info # Optional: Adjust logging levels
    networks:
      - posix_network

  peer2:
    build:
      context: .
      dockerfile: Dockerfile.peer # Reuse the peer Dockerfile for peer2
    container_name: async_posix_peer2
    restart: unless-stopped
    depends_on:
      - gateway
    environment:
      - RUST_LOG=info # Optional: Adjust logging levels
    networks:
      - posix_network

networks:
  posix_network:
    driver: bridge
```

Next build the Docker container images using Docker Compose.

```shell
# Build the Docker images
docker-compose build

# Start the services
docker-compose up
```
