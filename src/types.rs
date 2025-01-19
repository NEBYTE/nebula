/// Nebula data types.
/// Address, Transaction, BlockHeader, Block.
use serde::{Serialize, Deserialize};

pub type Address = [u8; 32];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub parent_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub validator: Address,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
