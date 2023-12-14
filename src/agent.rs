use crate::block_header::BlockHeader;
use crate::DIFFICULTY_TARGET;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{error, info};

/// The [`Agent`] is responsible for connecting to the [`Coordinator`],
/// listening for and generating new [`BlockHeader`]s
pub struct Agent;

impl Agent {
    /// Start the [`Agent`] which connects to the [`Coordinator`] and listens for new [`BlockHeader`]s
    pub async fn start(&mut self, coordinator_address: &str) {
        match TcpStream::connect(coordinator_address).await {
            Ok(mut stream) => {
                info!("Successfully connected to coordinator at {}", coordinator_address);
                loop {
                    if let Err(e) = self.listen_for_block_header(&mut stream).await {
                        error!("Error while listening for block header: {:?}", e);
                        // Decide what to do on error: break, continue, or reconnect logic
                        break;
                    }
                }
            }
            Err(e) => {
                error!("Failed to connect to coordinator at {}: {:?}", coordinator_address, e);
                // Optional: Implement retry logic here if needed
            }
        }
    }

    /// Reads the exact expected byte length of an incoming message (`108` bytes)
    /// and then sends a new [`BlockHeader`] to the [`Coordinator`] if it finds a hash which satisfies the difficulty target
    async fn listen_for_block_header(
        &mut self,
        stream: &mut TcpStream,
    ) -> Result<(), std::io::Error> {
        let mut buffer = [0u8; 108];

        stream.read_exact(&mut buffer).await?;
        info!("Received new block header from coordinator");
        let block_header = BlockHeader::from_bytes(&buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let new_block_header = self.mine_block(block_header).unwrap();
        let serialized = new_block_header.serialize();
        stream.write_all(&serialized).await?;
        Ok(())
    }

    /// Mines a new block header with the given block header as the previous block hash.
    /// It is using the [`hash`] function from the [`BlockHeader`]
    fn mine_block(&self, mut block_header: BlockHeader) -> Option<BlockHeader> {
        while block_header.nonce < u32::MAX {
            if block_header.is_less_than_difficulty() {
                return Some(BlockHeader {
                    version: 0x04,
                    prev_block_hash: block_header.prev_block_hash,
                    merkle_root: block_header.merkle_root,
                    time: block_header.time,
                    difficulty: DIFFICULTY_TARGET,
                    nonce: block_header.nonce,
                });
            } else {
                block_header.nonce += 1;
            }
        }

        None
    }
}
