use std::collections::HashMap;
use std::sync::Arc;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use crate::core::types::{DbWrapper, MutexWrapper, Neuron};

#[derive(Serialize, Deserialize, Clone)]
pub struct StakingModule {
    pub neurons: Arc<MutexWrapper<HashMap<u64, Neuron>>>,
    #[serde(skip)]
    pub db: DbWrapper,
}

impl StakingModule {
    pub fn new(neurons: Arc<MutexWrapper<HashMap<u64, Neuron>>>, db: Arc<DB>) -> Self {
        let module = Self {
            neurons,
            db: DbWrapper(db),
        };

        module.load_state();
        module
    }

    pub fn load_state(&self) {
        let mut stored_neurons = self.neurons.lock();
        stored_neurons.clear();

        let db_neurons = self.db.0.iterator(rocksdb::IteratorMode::Start);
        for item in db_neurons {
            let (key, value) = item.unwrap();

            if key.starts_with(b"neuron_") && key.len() == b"neuron_".len() + 8 {
                let id_bytes = &key[b"neuron_".len()..];
                let key_u64 = u64::from_le_bytes(id_bytes.try_into().unwrap());
                match bincode::deserialize::<Neuron>(&value) {
                    Ok(neuron) => {
                        stored_neurons.insert(key_u64, neuron);
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize neuron with key {}: {}", key_u64, e);
                    }
                }
            }
        }
    }
}
