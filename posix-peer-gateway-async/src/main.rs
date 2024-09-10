use std::sync::Arc;
use svckit::{
    fullduplex_gateway::FullDuplexPeerGateway, fullduplex_peer::FullDuplexPeer,
    fullduplexer::FullDuplexer,
};
use tokio::io::{AsyncReadExt, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Load the blacklist from the .env file and wrap it in Arc for shared access
    let blacklist = FullDuplexPeerGateway::load_blacklist();
    let blacklist = Arc::new(blacklist);

    // Create the PeerGateway with the shared blacklist
    let mut gateway = FullDuplexPeerGateway::new(blacklist.clone());

    // Create two peers with FIFO paths
    let peer1 = FullDuplexPeer::new("peer1", "/tmp/peer1_read_fifo", "/tmp/peer1_write_fifo");
    let peer2 = FullDuplexPeer::new("peer2", "/tmp/peer2_read_fifo", "/tmp/peer2_write_fifo");

    // Add peers to the gateway
    gateway.add_peer(peer1).await;
    gateway.add_peer(peer2).await;

    // Simulated peer communication (read from stdin, send to gateway)
    let mut input = tokio::io::stdin();
    let mut buffer = vec![0; 128];

    println!("Enter a message to send from peer1 to peer2:");

    // Read input from stdin
    let n = input.read(&mut buffer).await?;
    buffer.truncate(n);

    // Send the message from peer1 to peer2 via the gateway
    if let Err(e) = gateway
        .send(&svckit::Context {}, &mut &buffer[..], n as i64)
        .await
    {
        eprintln!("Failed to send message: {}", e);
    } else {
        println!("Message sent successfully!");
    }

    // Receive the message at peer2
    let mut output = Vec::new();
    if let Err(e) = gateway
        .receive(&svckit::Context {}, &mut output, n as i64)
        .await
    {
        eprintln!("Failed to receive message: {}", e);
    } else {
        println!("Peer2 received: {}", String::from_utf8_lossy(&output));
    }

    Ok(())
}
