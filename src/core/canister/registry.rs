use crate::core::canister::canister::{Canister};
use crate::core::types::{DbWrapper, MutexWrapper};
use std::collections::HashMap;
use std::sync::Arc;
use rocksdb::DB;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CanisterRegistry {
    pub canisters: Arc<MutexWrapper<HashMap<String, Arc<MutexWrapper<Canister>>>>>,
    #[serde(skip)]
    pub db: DbWrapper,
}

impl CanisterRegistry {
    pub fn new(db: Arc<DB>) -> Self {
        let mut registry = HashMap::new();
        let stored_canisters = db.iterator(rocksdb::IteratorMode::Start);
        for item in stored_canisters {
            let (key, value) = item.unwrap();
            if key.starts_with(b"canister_") {
                if let Ok(canister) = bincode::deserialize::<Canister>(&value) {
                    registry.insert(String::from_utf8(key.to_vec()).unwrap(), Arc::new(MutexWrapper::new(canister)));
                }
            }
        }
        Self {
            canisters: Arc::new(MutexWrapper::new(registry)),
            db: DbWrapper(db),
        }
    }

    pub fn register_canister(&mut self, canister_id: &String, canister: Canister) {
        let mut registry = self.canisters.lock();
        let serialized = bincode::serialize(&canister).unwrap();
        let key = format!("canister_{}", canister_id);
        self.db.put(key.as_bytes(), serialized).unwrap();
        registry.insert(canister_id.clone(), Arc::new(MutexWrapper::new(canister)));
    }

    pub fn get_canister(&self, canister_id: &str) -> Option<Canister> {
        let key = format!("canister_{}", canister_id);
        if let Ok(Some(data)) = self.db.get(key.as_bytes()) {
            if let Ok(canister) = bincode::deserialize::<Canister>(&data) {
                return Some(canister);
            }
        }
        None
    }
}
