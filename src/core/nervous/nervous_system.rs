use std::collections::HashMap;
use std::sync::Arc;
use rocksdb::DB;
use serde::{Deserialize, Serialize};

use crate::core::types::{Neuron, MutexWrapper, DbWrapper};

#[derive(Serialize, Deserialize, Clone)]
pub struct NervousSystem {
    pub neurons: Arc<MutexWrapper<HashMap<u64, Neuron>>>,
    pub next_id: Arc<MutexWrapper<u64>>,
    #[serde(skip)]
    pub db: DbWrapper,
}

impl NervousSystem {
    pub fn new(db: Arc<DB>) -> Self {
        let system = Self {
            neurons: Arc::new(MutexWrapper::new(HashMap::new())),
            next_id: Arc::new(MutexWrapper::new(1)),
            db: DbWrapper(db),
        };

        system.load_state();
        system
    }

    pub fn persist_neurons(&self) {
        let neurons = self.neurons.lock();
        for (id, neuron) in neurons.iter() {
            let serialized = bincode::serialize(neuron).unwrap();
            let mut key: Vec<u8> = b"neuron_".to_vec();
            key.extend_from_slice(&id.to_le_bytes());
            self.db.put(key, serialized).unwrap();
        }
    }

    fn load_state(&self) {
        let mut neurons_lock = self.neurons.lock();
        neurons_lock.clear();

        let iter = self.db.0.iterator(rocksdb::IteratorMode::Start);
        for item in iter {
            let (key, value) = item.unwrap();

            if key.starts_with(b"neuron_") && key.len() == b"neuron_".len() + 8 {
                let id_bytes = &key[b"neuron_".len()..];
                let key_u64 = u64::from_le_bytes(id_bytes.try_into().expect("Failed to convert key to u64"));
                match bincode::deserialize::<Neuron>(&value) {
                    Ok(neuron) => {
                        neurons_lock.insert(key_u64, neuron);
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize neuron ID {}: {}", key_u64, e);
                    }
                }
            }
        }
    }
}

