use crate::block_header::BlockHeader;
use crate::blockchain::Blockchain;
use crate::BLOCK_TIME_IN_SECONDS;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::info;

/// The [`Coordinator`] is responsible for listening for new [`BlockHeader`]s from the [`Agent`]s
#[derive(Clone)]
pub struct Coordinator {
    /// The [`Agent`]s which are connected to the [`Coordinator`]
    agents: Arc<Mutex<Vec<WriteHalf<TcpStream>>>>,
    /// The [`Blockchain`] which stores all [`BlockHeader`]s
    blockchain: Arc<Mutex<Blockchain>>,
}
impl Coordinator {
    /// Creates a new [`Coordinator`]
    pub fn new() -> Self {
        Coordinator {
            agents: Default::default(),
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
        }
    }

    /// Runs the [`Coordinator`] on the given address
    pub async fn run(&self, address: &str) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(address).await?;

        while let Ok((socket, _)) = listener.accept().await {
            let (reader, writer) = tokio::io::split(socket);
            self.agents.lock().await.push(writer);

            let blockchain_clone = self.blockchain.clone();

            tokio::spawn(async move {
                Coordinator::handle_agent(reader, blockchain_clone).await;
            });
        }

        Ok(())
    }

    /// Handles the [`Agent`] by reading new [`BlockHeader`]s and broadcasting the latest [`BlockHeader`] to all [`Agent`]s
    async fn handle_agent(
        mut reader: ReadHalf<TcpStream>,
        blockchain: Arc<Mutex<Blockchain>>,
    ) {
        let mut buffer = [0u8; 108];

        loop {
            if reader.read_exact(&mut buffer).await.is_ok() {
                let received_block_header = BlockHeader::from_bytes(&buffer).unwrap();
                let latest_block = blockchain.lock().await.get_current_block();

                if received_block_header == latest_block {
                    info!("Block already received, ignoring");
                    continue;
                }

                info!("COORDINATOR: RECEIVED BLOCK: {:?}", received_block_header);
                if received_block_header.is_less_than_difficulty() {
                    let mut blockchain = blockchain.lock().await;
                    blockchain.update(received_block_header.clone());
                } else {
                    info!("COORDINATOR: BLOCK NOT VALID: {:?}", received_block_header);
                }
            }
        }
    }

    /// Broadcasts the given [`BlockHeader`] to all connected [`Agent`]s
    pub async fn broadcast_new_blocks(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(BLOCK_TIME_IN_SECONDS));

        loop {
            interval.tick().await;
            let latest_block = self.blockchain.lock().await.get_current_block();
            let new_block = BlockHeader::new(latest_block.hash());

            for agent in self.agents.lock().await.iter_mut() {
                let serialized = new_block.serialize();
                let _ = agent.write_all(&serialized).await;
            }
        }
    }
}
