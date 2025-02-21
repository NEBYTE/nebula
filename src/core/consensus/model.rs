use std::collections::HashMap;
use std::sync::Arc;
use ed25519_dalek::VerifyingKey;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use crate::core::consensus::ValidatorInfo;
use crate::core::types::{Block, DbWrapper, MutexWrapper, Neuron, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub address: String,
    pub public_key: VerifyingKey,
    pub balance: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConsensusEngine {
    pub validators: Arc<MutexWrapper<Vec<ValidatorInfo>>>,
    pub neurons: Arc<MutexWrapper<HashMap<u64, Neuron>>>,
    pub mempool: Arc<MutexWrapper<Vec<Transaction>>>,
    pub chain: Arc<MutexWrapper<Vec<Block>>>,
    pub ledger: Arc<MutexWrapper<HashMap<String, Account>>>,
    #[serde(skip)]
    pub db: DbWrapper,
}

impl ConsensusEngine {
    pub fn new(validators: Arc<MutexWrapper<Vec<ValidatorInfo>>>, neurons: Arc<MutexWrapper<HashMap<u64, Neuron>>>, db: Arc<DB>) -> Self {
        let mut engine = Self {
            validators,
            neurons,
            mempool: Arc::new(MutexWrapper::new(Vec::new())),
            chain: Arc::new(MutexWrapper::new(Vec::new())),
            ledger: Arc::new(MutexWrapper::new(HashMap::new())),
            db: DbWrapper(db),
        };

        engine.load_state();
        engine
    }

    pub fn persist_state(&self) {
        {
            let ledger = self.ledger.lock();
            for (address, account) in ledger.iter() {
                let serialized = bincode::serialize(account).unwrap();
                let key = format!("ledger_{}", address);
                self.db.put(key.as_bytes(), serialized).unwrap();
            }
            drop(ledger);
        }

        {
            let chain = self.chain.lock();
            for (i, block) in chain.iter().enumerate() {
                let serialized = bincode::serialize(block).unwrap();
                let key = format!("block_{}", i);
                self.db.put(key.as_bytes(), serialized).unwrap();
            }
            drop(chain);
        }

        {
            let mempool = self.mempool.lock();
            for (i, tx) in mempool.iter().enumerate() {
                let serialized = bincode::serialize(tx).unwrap();
                let key = format!("mempool_{}", i);
                self.db.put(key.as_bytes(), serialized).unwrap();
            }
            drop(mempool);
        }

        {
            let validators = self.validators.lock();
            let serialized = bincode::serialize(&*validators).unwrap();
            self.db.put(b"validators", serialized).unwrap();
            drop(validators);
        }
    }


    fn load_state(&mut self) {
        {
            let mut ledger_lock = self.ledger.lock();
            ledger_lock.clear();
            let iter = self.db.0.iterator(rocksdb::IteratorMode::Start);
            for item in iter {
                let (key, value) = item.unwrap();
                let key_str = String::from_utf8(key.to_vec()).unwrap();
                if let Ok(account) = bincode::deserialize::<Account>(&value) {
                    ledger_lock.insert(key_str, account);
                }
            }
            drop(ledger_lock)
        }

        {
            let mut chain_lock = self.chain.lock();
            chain_lock.clear();
            for i in 0.. {
                let key = format!("block_{}", i);
                if let Ok(Some(value)) = self.db.get(&key) {
                    if let Ok(block) = bincode::deserialize::<Block>(&value) {
                        chain_lock.push(block);
                    }
                } else {
                    break;
                }
            }
            drop(chain_lock)
        }

        {
            let mut mempool_lock = self.mempool.lock();
            mempool_lock.clear();
            for i in 0.. {
                let key = format!("mempool_{}", i);
                if let Ok(Some(value)) = self.db.get(&key) {
                    if let Ok(tx) = bincode::deserialize::<Transaction>(&value) {
                        println!("Loaded transaction {}", i);
                        mempool_lock.push(tx);
                    }
                } else {
                    break;
                }
            }
            drop(mempool_lock)
        }

        {
            let mut validators_lock = self.validators.lock();
            if let Ok(Some(value)) = self.db.get("validators") {
                if let Ok(validators) = bincode::deserialize::<Vec<ValidatorInfo>>(&value) {
                    *validators_lock = validators;
                }
            }
            drop(validators_lock)
        }
    }

    pub fn init_ledger(&mut self, address: String, public_key: VerifyingKey, balance: u64) -> Option<Account> {
        let account = Account {
            address: address.clone(),
            public_key,
            balance,
        };

        {
            let mut ledger = self.ledger.lock();
            ledger.insert(address.clone(), account);
        }

        self.persist_state();

        let ledger = self.ledger.lock();
        ledger.get(&address).cloned()
    }

    pub fn get_ledger(&mut self, address: String) -> Option<Account> {
        let ledger = self.ledger.lock();
        let result = ledger.get(&address).cloned();
        result
    }

    pub fn get_balance(&mut self, address: &str) -> u64 {
        let ledger = self.ledger.lock();
        let balance = ledger.get(address).map(|account| account.balance).unwrap_or(0);
        balance
    }
}
