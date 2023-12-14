use std::cmp::Ordering;
use crate::DIFFICULTY_TARGET;
use digest::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

/// The [`BlockHeader`] is the header of a block in the blockchain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    /// The version of the block header
    pub(crate) version: i32,
    /// The hash of the previous block header
    pub(crate) prev_block_hash: [u8; 32],
    /// The merkle root of the transactions
    pub(crate) merkle_root: [u8; 32],
    /// The time of the block header
    pub(crate) time: u32,
    /// The difficulty target of the block header
    pub(crate) difficulty: [u8; 32],
    /// The nonce of the block header
    pub(crate) nonce: u32,
}

impl BlockHeader {
    /// Creates a new [`BlockHeader`] with the given previous block hash
    pub fn new(prev_block_hash: [u8; 32]) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        let merkle_root = [0u8; 32];

        BlockHeader {
            version: 0x04,
            prev_block_hash,
            merkle_root,
            time,
            difficulty: DIFFICULTY_TARGET,
            nonce: 0,
        }
    }

    /// Hashes the serialized [`BlockHeader`] twice with the sha256 algorithm
    pub fn hash(&self) -> [u8; 32] {
        let serialized = self.serialize();
        sha2::Sha256::digest(sha2::Sha256::digest(serialized)).into()
    }

    /// Serializes the [`BlockHeader`] into a byte array
    pub fn serialize(&self) -> [u8; 108] {
        let mut buffer = [0u8; 108];
        buffer[..4].copy_from_slice(&self.version.to_le_bytes());
        buffer[4..36].copy_from_slice(&self.prev_block_hash);
        buffer[36..68].copy_from_slice(&self.merkle_root);
        buffer[68..72].copy_from_slice(&self.time.to_le_bytes());
        buffer[72..104].copy_from_slice(&self.difficulty);
        buffer[104..108].copy_from_slice(&self.nonce.to_le_bytes());
        buffer
    }

    /// Returns a [`BlockHeader`] from a serialized byte array
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 108 {
            return Err("Invalid byte length for BlockHeader");
        }

        let version = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let prev_block_hash = bytes[4..36].try_into().unwrap();
        let merkle_root = bytes[36..68].try_into().unwrap();
        let time = u32::from_le_bytes(bytes[68..72].try_into().unwrap());
        let difficulty = bytes[72..104].try_into().unwrap();
        let nonce = u32::from_le_bytes(bytes[104..108].try_into().unwrap());

        Ok(BlockHeader {
            version,
            prev_block_hash,
            merkle_root,
            time,
            difficulty,
            nonce,
        })
    }

    /// Checks if the given hash is less than the given difficulty
    pub fn is_less_than_difficulty(&self) -> bool {
        let hash = self.hash();

        for (h, d) in hash.iter().zip(DIFFICULTY_TARGET.iter()) {
            match h.cmp(d) {
                Ordering::Less => return true,
                Ordering::Greater => return false,
                Ordering::Equal => continue,
            }
        }

        true
    }
}
