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

## The API for the Full Duplexer FIFO Async Peer Gateway

The core of the API is provided in the `svckit` crate referencing the the following source files.

- fullduplexer-async.rs - This is the Rust trait that defines the async send*/receive* APIs that the `FullDuplexPeer`
and the `FullDuplexPeerGateway`implement.

The trait API for `FullDuplexer` is defined as follows.

```rust
use tokio::io::{AsyncRead, AsyncWrite, Result};

// Define a context struct, if needed (you may expand this as per your requirements)
pub struct Context;

#[async_trait::async_trait]
pub trait FullDuplexer {
    // Asynchronous send method
    async fn send(
        &self,
        ctx: &Context,
        reader: &mut (dyn AsyncRead + Unpin),
        n: i64,
    ) -> Result<usize>;

    // Asynchronous send_all method to handle multiple readers
    async fn send_all(
        &self,
        ctx: &Context,
        readers: Vec<&mut (dyn AsyncRead + Unpin)>,
        n: i64,
    ) -> Result<usize> {
        let mut total_bytes = 0;
        for reader in readers {
            total_bytes += self.send(ctx, reader, n).await?;
        }
        Ok(total_bytes)
    }

    // Asynchronous receive method
    async fn receive(
        &self,
        ctx: &Context,
        writer: &mut (dyn AsyncWrite + Unpin),
        n: i64,
    ) -> Result<usize>;

    // Asynchronous receive_all method to handle multiple writers
    async fn receive_all(
        &self,
        ctx: &Context,
        writers: Vec<&mut (dyn AsyncWrite + Unpin)>,
        n: i64,
    ) -> Result<usize> {
        let mut total_bytes = 0;
        for writer in writers {
            total_bytes += self.receive(ctx, writer, n).await?;
        }
        Ok(total_bytes)
    }
}
```


- fullduplexer-async-peer.rs
- fullduplexer-async-peer-gateway.rs



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
