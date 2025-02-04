use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use ed25519_dalek::SigningKey;
use crate::types::{Neuron, NeuronStatus};

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

    pub fn create_neuron(&self, caller: &SigningKey, name: String, dissolve_days: i64) -> Result<u64, String> {
        let now = Utc::now();
        let neuron_id;

        {
            let mut id_counter = self.next_id.lock().unwrap();
            neuron_id = *id_counter;
            *id_counter += 1;
        }

        let neuron = Neuron {
            name,
            visibility: true,
            id: neuron_id,
            private_address: Arc::new(caller.clone()),
            state: NeuronStatus::NotDissolving,
            staked: false,
            staked_amount: 0,
            dissolve_days: now.date_naive() + chrono::Duration::days(dissolve_days),
            age: now.date_naive(),
            voting_power: 0,
            date_created: now,
            dissolve_delay_bonus: 0,
            age_bonus: 0,
            total_bonus: 0,
            is_genesis: false,
            is_known_neuron: false,
            validator: None,
        };

        let mut neurons = self.neurons.lock().unwrap();
        neurons.insert(neuron_id, neuron);

        Ok(neuron_id)
    }

    pub fn get_neuron(&self, neuron_id: u64) -> Option<Neuron> {
        let neurons = self.neurons.lock().unwrap();
        neurons.get(&neuron_id).cloned()
    }

    pub fn list_neurons(&self) -> Vec<Neuron> {
        let neurons = self.neurons.lock().unwrap();
        neurons.values().cloned().collect()
    }
}
