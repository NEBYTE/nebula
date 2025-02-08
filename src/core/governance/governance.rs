use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, RwLock, Mutex};
use crate::core::consensus::math::voting_power;
use crate::core::types::{Neuron, Vote};
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
    pub fn compute_voting_outcome(&self, proposal: &Proposal) -> i64 {
        let neurons = self.neurons.lock().unwrap();
        let mut outcome: i64 = 0;

        for (&neuron_id, voting_neuron) in proposal.votes_of_neurons.iter() {
            let vote_value = match voting_neuron.vote {
                Vote::Yes => 1,
                Vote::No => -1,
                _ => 0,
            };
            if let Some(neuron) = neurons.get(&neuron_id) {
                let effective_power = voting_power(neuron.staked_amount, neuron.bonus_multiplier);
                outcome += (effective_power as i64) * vote_value;
            }
        }
        outcome
    }
}
