use super::FullDuplexer;
use libc::{open, O_CREAT, O_RDONLY, O_WRONLY};
use std::ffi::CString;
use std::os::unix::io::RawFd;
use tokio::fs::File;
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};

pub struct FullDuplexPeer {
    pub id: String,
    pub read_fifo: String,
    pub write_fifo: String,
}

impl FullDuplexPeer {
    pub fn new(id: &str, read_fifo: &str, write_fifo: &str) -> Self {
        unsafe {
            let read_path = CString::new(read_fifo).unwrap();
            let write_path = CString::new(write_fifo).unwrap();
            libc::mkfifo(read_path.as_ptr(), 0o644);
            libc::mkfifo(write_path.as_ptr(), 0o644);
        }

        Self {
            id: id.to_string(),
            read_fifo: read_fifo.to_string(),
            write_fifo: write_fifo.to_string(),
        }
    }

    async fn open_fifo(path: &str, flags: i32) -> Result<RawFd> {
        let c_path = CString::new(path).unwrap();
        let fd = unsafe { open(c_path.as_ptr(), flags | O_CREAT) };
        if fd < 0 {
            Err(std::io::Error::last_os_error().into())
        } else {
            Ok(fd)
        }
    }

    async fn send_posix(&self, buffer: &[u8]) -> Result<usize> {
        let fd = Self::open_fifo(&self.write_fifo, O_WRONLY).await?;
        let mut file = unsafe { File::from_raw_fd(fd) };
        file.write_all(buffer).await?;
        Ok(buffer.len())
    }

    async fn receive_posix(&self, buffer: &mut [u8]) -> Result<usize> {
        let fd = Self::open_fifo(&self.read_fifo, O_RDONLY).await?;
        let mut file = unsafe { File::from_raw_fd(fd) };
        let n = file.read_exact(buffer).await?;
        Ok(n)
    }
}

// Implement the FullDuplexer trait for FullDuplexPeer using async/await
#[async_trait::async_trait]
impl FullDuplexer for FullDuplexPeer {
    async fn send(
        &self,
        _ctx: &Context,
        reader: &mut (dyn AsyncRead + Unpin),
        n: i64,
    ) -> Result<usize> {
        let mut buffer = vec![0; n as usize];
        reader.read_exact(&mut buffer).await?;
        self.send_posix(&buffer).await
    }

    async fn send_all(
        &self,
        _ctx: &Context,
        readers: Vec<&mut (dyn AsyncRead + Unpin)>,
        n: i64,
    ) -> Result<usize> {
        let mut total_bytes = 0;
        for reader in readers {
            total_bytes += self.send(_ctx, reader, n).await?;
        }
        Ok(total_bytes)
    }

    async fn receive(
        &self,
        _ctx: &Context,
        writer: &mut (dyn AsyncWrite + Unpin),
        n: i64,
    ) -> Result<usize> {
        let mut buffer = vec![0; n as usize];
        self.receive_posix(&mut buffer).await?;
        writer.write_all(&buffer).await?;
        Ok(buffer.len())
    }

    async fn receive_all(
        &self,
        _ctx: &Context,
        writers: Vec<&mut (dyn AsyncWrite + Unpin)>,
        n: i64,
    ) -> Result<usize> {
        let mut total_bytes = 0;
        for writer in writers {
            total_bytes += self.receive(_ctx, writer, n).await?;
        }
        Ok(total_bytes)
    }
}
