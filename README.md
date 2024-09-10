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
├── src/
│   └── main.rs
├── svckit/
│   ├── lib.rs
│   ├── fullduplexer.rs
│   ├── fullduplex-peer.rs
│   └── fullduplex-gateway.rs
```


## Build Docker Images and Run

```shell
# Build the Docker images
docker-compose build

# Start the services
docker-compose up
```
