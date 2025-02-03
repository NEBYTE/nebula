use serde::{Serialize, Deserialize};

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
    name: String,
    id: u64,
    vote: Vote,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Neuron {
    name: String,
    id: u64,
    state: NeuronStatus,
    staked: bool,
    staked_amount: usize,
    dissolve_days: chrono::prelude::NaiveDate,
    age: chrono::prelude::NaiveDate,
    voting_power: u32,
    date_created: chrono::DateTime<chrono::prelude::Utc>,
    dissolve_delay_bonus: u32,
    age_bonus: u32,
    total_bonus: u32,
    is_genesis: bool,
    is_known_neuron: bool,
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
