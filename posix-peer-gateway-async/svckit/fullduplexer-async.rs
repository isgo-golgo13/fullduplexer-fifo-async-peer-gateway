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
