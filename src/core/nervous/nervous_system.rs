use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::core::types::Neuron;

pub struct NervousSystem {
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
    pub next_id: Arc<Mutex<u64>>,
}

impl NervousSystem {
    pub fn new() -> Self {
        Self {
            neurons: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

