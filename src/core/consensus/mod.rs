pub mod validator;
pub mod transaction;
pub mod block;
pub mod neuron;
pub mod utils;

use crate::core::types::{Block, Transaction, Neuron};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub struct ConsensusEngine {
    pub validators: Vec<ValidatorInfo>,
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
    pub mempool: Vec<Transaction>,
    pub chain: Vec<Block>,
}

impl ConsensusEngine {
    pub fn new(validators: Vec<ValidatorInfo>, neurons: Arc<Mutex<HashMap<u64, Neuron>>>) -> Self {
        Self {
            validators,
            neurons,
            mempool: Vec::new(),
            chain: Vec::new(),
        }
    }
}

pub use validator::{ValidatorInfo, select_next_validator, slash};
pub use transaction::{add_transaction, compute_transaction_hash};
pub use block::{produce_block, validate_block, compute_merkle_root, hash_block, serialize_header_for_signing};
pub use neuron::delegate_stake;
pub use utils::crypto_hash;
