use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use chrono::Utc;
use ed25519_dalek::SigningKey;
use crate::types::{Neuron};
pub struct StakingModule {
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
}

impl StakingModule {
    pub fn new(neurons: Arc<Mutex<HashMap<u64, Neuron>>>) -> Self {
        Self {
            neurons
        }
    }

    pub fn stake(&mut self, caller: &SigningKey, neuron_id: u64, amount: u64) -> Result<(), String> {
        let mut neurons = self.neurons.lock().unwrap();
        let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

        if neuron.private_address != Arc::new(caller.clone()) {
            return Err("Caller is not the owner of this neuron".to_string());
        }

        neuron.staked = true;
        neuron.staked_amount += amount;

        Ok(())
    }

    pub fn unstake(&mut self, caller: &SigningKey, neuron_id: u64, amount: u64) -> Result<(), String> {
        let mut neurons = self.neurons.lock().unwrap();
        let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

        if neuron.private_address != Arc::new(caller.clone()) {
            return Err("Caller is not the owner of this neuron".to_string());
        }

        if neuron.unlock_date > Utc::now().date_naive() {
            return Err("Neuron is locked in dissolve delay".to_string());
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
            let maturity_bonus = 1.0 + (neuron.maturity as f64 / 100.0);
            let reward = ((reward_pool as f64 * ratio) * maturity_bonus) as u64;
            neuron.staked_amount += reward;
            neuron.maturity += 1;
        }
    }
}
