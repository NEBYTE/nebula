use std::sync::{Arc};
use chrono::Utc;
use ed25519_dalek::{SigningKey, VerifyingKey};
use crate::core::types::{Neuron, NeuronStatus};
use crate::core::nervous::nervous_system::NervousSystem;

pub fn create_neuron(
    nervous_system: &mut NervousSystem,
    caller: &SigningKey,
    name: String,
    dissolve_days: i64
) -> Result<u64, String> {
    let now = Utc::now();
    let neuron_id;
    {
        let mut id_counter = nervous_system.next_id.lock();
        neuron_id = *id_counter;
        *id_counter += 1;
    }

    let validator_address = hex::encode(VerifyingKey::from(caller).to_bytes());

    let neuron = Neuron {
        name,
        address: hex::encode(VerifyingKey::from(caller).to_bytes()),
        visibility: true,
        id: neuron_id,
        private_address: Arc::new(caller.clone()),
        state: NeuronStatus::NotDissolving,
        staked: false,
        staked_amount: 0,
        unlock_date: now.date_naive() + chrono::Duration::days(dissolve_days),
        age: now.date_naive(),
        voting_power: 0,
        maturity: 0,
        bonus_multiplier: 1.0,
        date_created: now,
        dissolve_delay_bonus: 0,
        age_bonus: 0,
        total_bonus: 0,
        is_genesis: false,
        is_known_neuron: false,
        validator: Some(validator_address.clone()),
    };

    {
        let mut neurons = nervous_system.neurons.lock();
        neurons.insert(neuron_id, neuron);
    }

    nervous_system.persist_neurons();
    Ok(neuron_id)
}

pub fn get_neuron(
    nervous_system: &mut NervousSystem,
    neuron_id: u64
) -> Option<Neuron> {
    let neurons = nervous_system.neurons.lock();
    neurons.get(&neuron_id).cloned()
}

pub fn list_neurons(
    nervous_system: &mut NervousSystem
) -> Vec<Neuron> {
    let neurons = nervous_system.neurons.lock();
    neurons.values().cloned().collect()
}