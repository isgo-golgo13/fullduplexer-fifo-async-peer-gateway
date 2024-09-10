use super::{Context, FullDuplexPeer, FullDuplexer};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};

pub struct FullDuplexPeerGateway {
    active_peers: Arc<HashMap<String, FullDuplexPeer>>,
    inactive_peers: Arc<HashMap<String, FullDuplexPeer>>,
    blacklist: Arc<Vec<String>>, // Blacklist wrapped in Arc for async version
}

impl FullDuplexPeerGateway {
    pub fn new(blacklist: Arc<Vec<String>>) -> Self {
        Self {
            active_peers: Arc::new(HashMap::new()),
            inactive_peers: Arc::new(HashMap::new()),
            blacklist,
        }
    }

    pub fn load_blacklist() -> Arc<Vec<String>> {
        dotenv::dotenv().ok(); // Load .env file
        let blacklist_str = std::env::var("BLACKLIST").unwrap_or_default();
        let blacklist: Vec<String> = blacklist_str
            .split(',')
            .map(|term| term.trim().to_string())
            .collect();
        Arc::new(blacklist)
    }

    pub fn filter_blacklisted_content(&self, message: &str) -> bool {
        for term in self.blacklist.iter() {
            if message.contains(term) {
                return true; // Blacklisted content found
            }
        }
        false
    }

    pub async fn add_peer(&mut self, peer: FullDuplexPeer) {
        Arc::get_mut(&mut self.active_peers)
            .unwrap()
            .insert(peer.id.clone(), peer);
    }

    pub async fn remove_peer(&mut self, peer_id: &str) {
        if let Some(peer) = Arc::get_mut(&mut self.active_peers)
            .unwrap()
            .remove(peer_id)
        {
            Arc::get_mut(&mut self.inactive_peers)
                .unwrap()
                .insert(peer_id.to_string(), peer);
        }
    }

    pub async fn send(
        &self,
        ctx: &Context,
        mut reader: impl tokio::io::AsyncRead + Unpin,
        n: i64,
    ) -> Result<usize> {
        let mut buffer = vec![0; n as usize];
        reader.read_exact(&mut buffer).await?;
        let message = String::from_utf8_lossy(&buffer);

        if self.filter_blacklisted_content(&message) {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::InvalidInput,
                "Message contains blacklisted terms",
            ));
        }

        for peer in self.active_peers.values() {
            peer.send(ctx, &mut buffer.as_slice(), n).await?;
        }
        Ok(n as usize)
    }

    pub async fn receive(
        &self,
        ctx: &Context,
        mut writer: impl tokio::io::AsyncWrite + Unpin,
        n: i64,
    ) -> Result<usize> {
        for peer in self.active_peers.values() {
            peer.receive(ctx, &mut writer, n).await?;
        }
        Ok(n as usize)
    }
}

// Implement the FullDuplexer trait for FullDuplexPeerGateway
impl FullDuplexer for FullDuplexPeerGateway {
    async fn send(
        &self,
        ctx: &Context,
        reader: &mut (dyn tokio::io::AsyncRead + Unpin),
        n: i64,
    ) -> Result<usize> {
        self.send(ctx, reader, n).await
    }

    async fn receive(
        &self,
        ctx: &Context,
        writer: &mut (dyn tokio::io::AsyncWrite + Unpin),
        n: i64,
    ) -> Result<usize> {
        self.receive(ctx, writer, n).await
    }
}
