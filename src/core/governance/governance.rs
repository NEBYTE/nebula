use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, RwLock, Mutex};
use crate::core::types::Neuron;
use crate::core::governance::proposal::Proposal;

pub struct Governance {
    pub proposals: Arc<RwLock<BinaryHeap<Proposal>>>,
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
    pub total_voting_power: Arc<Mutex<u128>>,
    pub daily_voting_rewards: Arc<Mutex<u128>>,
    pub next_id: Arc<Mutex<u64>>,
}

impl Governance {
    pub fn new(neurons: Arc<Mutex<HashMap<u64, Neuron>>>) -> Self {
        Self {
            proposals: Arc::new(RwLock::new(BinaryHeap::new())),
            neurons,
            total_voting_power: Arc::new(Mutex::new(500_000_000)),
            daily_voting_rewards: Arc::new(Mutex::new(90_500)),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}
