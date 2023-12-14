use crate::block_header::BlockHeader;
use std::sync::{Arc, Mutex};

/// The [`Blockchain`] is responsible for storing the current block header and all previous block headers
#[derive(Clone)]
pub struct Blockchain {
    /// The current [`BlockHeader`]
    pub current_block: Arc<Mutex<BlockHeader>>,
    /// All previous [`BlockHeader`]s
    pub blocks: Arc<Mutex<Vec<BlockHeader>>>,
}

impl Blockchain {
    /// Creates a new [`Blockchain`] with a genesis block
    pub fn new() -> Self {
        let genesis_block = BlockHeader::new([0u8; 32]);

        Blockchain {
            current_block: Arc::new(Mutex::new(genesis_block)),
            blocks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Updates the current [`BlockHeader`] and adds it to the list of previous [`BlockHeader`]s
    pub fn update(&mut self, new_block_header: BlockHeader) {
        let mut block_header = self.current_block.lock().unwrap();
        *block_header = new_block_header.clone();
        self.blocks
            .lock()
            .expect("Cannot get lock for the blocks")
            .push(new_block_header);
    }

    /// Returns the latest [`BlockHeader`]
    pub fn get_current_block(&self) -> BlockHeader {
        self.current_block.lock().unwrap().clone()
    }
}
