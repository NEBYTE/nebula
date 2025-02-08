use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ed25519_dalek::VerifyingKey;
use crate::core::consensus::ValidatorInfo;
use crate::core::types::{Block, Neuron, Transaction};

#[derive(Clone, Debug)]
pub struct Account {
    pub address: String,
    pub public_key: VerifyingKey,
    pub balance: u64,
}

#[derive(Clone)]
pub struct ConsensusEngine {
    pub validators: Arc<Mutex<Vec<ValidatorInfo>>>,
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
    pub mempool:Arc<Mutex<Vec<Transaction>>>,
    pub chain: Arc<Mutex<Vec<Block>>>,
    pub ledger: Arc<Mutex<HashMap<String, Account>>>,
}

impl ConsensusEngine {
    pub fn new(validators: Arc<Mutex<Vec<ValidatorInfo>>>, neurons: Arc<Mutex<HashMap<u64, Neuron>>>) -> Self {
        Self {
            validators,
            neurons,
            mempool: Arc::new(Mutex::new(Vec::new())),
            chain: Arc::new(Mutex::new(Vec::new())),
            ledger: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn init_ledger(&mut self, address: String, public_key: VerifyingKey, balance: u64) -> Option<Account>{
        let account = Account {
            address: address.clone(),
            public_key,
            balance,
        };

        let mut ledger = self.ledger.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        ledger.insert(address.clone(), account);
        ledger.get(&address).cloned()
    }

    pub fn get_ledger(&self, address: String) -> Option<Account> {
        let ledger = self.ledger.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        ledger.get(&address).cloned()
    }
    pub fn get_balance(&mut self, address: &str) -> u64 {
        let ledger = self.ledger.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        ledger.get(address).map(|account| account.balance).unwrap_or(0)
    }
}