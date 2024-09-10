use dotenv::dotenv;
use std::env;

pub struct FifoContext {
    pub id: String,
    pub read_fifo: String,
    pub write_fifo: String,
}

impl FifoContext {
    // Load the FifoContext from .env configuration
    pub fn from_env() -> Self {
        dotenv().ok(); // Load the .env file

        let id = env::var("PEER_ID").expect("PEER_ID is not set in .env file");
        let read_fifo = env::var("READ_FIFO").expect("READ_FIFO is not set in .env file");
        let write_fifo = env::var("WRITE_FIFO").expect("WRITE_FIFO is not set in .env file");

        FifoContext {
            id,
            read_fifo,
            write_fifo,
        }
    }
}
