use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct CanisterRegistry {
    canisters: Arc<Mutex<HashMap<String, Arc<Mutex<Canister>>>>>,
}

impl CanisterRegistry {
    pub fn new() -> Self {
        Self {
            canisters: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_canister(&mut self, canister_id: &String, canister: Canister) {
        let mut registry = self.canisters.lock().unwrap();
        registry.insert(canister_id.clone(), Arc::new(Mutex::new(canister)));
    }

    pub fn get_canister(&self, canister_id: &str) -> Option<Canister> {
        let registry = self.canisters.lock().unwrap();
        registry.get(canister_id).map(|arc_canister| {
            let canister_guard = arc_canister.lock().unwrap();
            (*canister_guard).clone()
        })
    }

    pub fn execute_function<'a>(
        &self,
        canister_id: &str,
        payload: CanisterFunctionPayload<'a>,
    ) -> Result<String, String> {
        let mut canister = self
            .get_canister(canister_id)
            .ok_or(format!("Canister '{}' not found", canister_id))?;

        canister.execute_function(payload)
    }
}
