use serde::{Serialize, Deserialize};
use tokio_rustls::rustls::PrivateKey;

pub type Address = [u8; 32];
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum VotingStatus {
    Open,
    Pending,
    Terminated,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum NeuronStatus {
    NotDissolving,
    Dissolving,
    Dissolved,
    Spawning
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Vote {
    None,
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct VotingNeuron {
    pub name: String,
    pub id: u64,
    pub vote: Vote,
    pub private_address: Address,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Neuron {
    pub private_address: Address,
    pub name: String,
    pub visibility: bool,
    pub id: u64,
    pub state: NeuronStatus,
    pub staked: bool,
    pub staked_amount: u64,
    pub dissolve_days: chrono::prelude::NaiveDate,
    pub age: chrono::prelude::NaiveDate,
    pub voting_power: u32,
    pub date_created: chrono::DateTime<chrono::prelude::Utc>,
    pub dissolve_delay_bonus: u32,
    pub age_bonus: u32,
    pub total_bonus: u32,
    pub is_genesis: bool,
    pub is_known_neuron: bool,
    pub validator: Option<Address>,
}
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
