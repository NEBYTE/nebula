use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, RwLock, Mutex};
use crate::types::{Address, Neuron};
pub struct StakingModule {
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
}

impl StakingModule {
    pub fn new() -> Self {
        Self {
            neurons: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn stake(&mut self, caller: Address, neuron_id: u64, amount: u64) -> Result<(), String> {
        let mut neurons = self.neurons.lock().unwrap();
        let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

        if neuron.private_address != caller {
            return Err("Caller is not the owner of this neuron".to_string());
        }

        neuron.staked = true;
        neuron.staked_amount += amount;

        Ok(())
    }

    pub fn unstake(&mut self, caller: Address, neuron_id: u64, amount: u64) -> Result<(), String> {
        let mut neurons = self.neurons.lock().unwrap();
        let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

        if neuron.private_address != caller {
            return Err("Caller is not the owner of this neuron".to_string());
        }

        if neuron.staked_amount < amount {
            return Err("Insufficient staked amount".to_string());
        }

        neuron.staked_amount -= amount;
        if neuron.staked_amount == 0 {
            neuron.staked = false;
        }

        Ok(())
    }

    pub fn distribute_rewards(&mut self, reward_pool: u64) {
        let mut neurons = self.neurons.lock().unwrap();
        let total_staked: u64 = neurons.values().map(|n| n.staked_amount).sum();

        if total_staked == 0 {
            return;
        }

        for neuron in neurons.values_mut() {
            let ratio = neuron.staked_amount as f64 / total_staked as f64;
            let reward = (reward_pool as f64 * ratio) as u64;
            neuron.staked_amount += reward;
        }
    }
}
