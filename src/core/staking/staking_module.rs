use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::core::types::Neuron;

pub struct StakingModule {
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
}

impl StakingModule {
    pub fn new(neurons: Arc<Mutex<HashMap<u64, Neuron>>>) -> Self {
        Self { neurons }
    }
}
