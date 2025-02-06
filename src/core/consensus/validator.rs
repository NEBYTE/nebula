use crate::core::types::{Address};
use sha2::{Digest, Sha256};
use chrono::Utc;
use crate::core::consensus::ConsensusEngine;

#[derive(Clone)]
pub struct ValidatorInfo {
    pub address: Address,
    pub active: bool,
}
pub fn slash(
    consensus_engine: &mut ConsensusEngine,
    validator: Address,
    amount: u64,
) {
    let penalty = amount * 2;
    let mut neurons = consensus_engine.neurons.lock().unwrap();

    for neuron in neurons.values_mut() {
        if neuron.validator == Some(validator.clone()) {
            if neuron.staked_amount > penalty {
                neuron.staked_amount -= penalty;
            } else {
                neuron.staked_amount = 0;
            }
        }
    }

    if let Some(v) = consensus_engine.validators.iter_mut().find(|v| v.address == validator) {
        v.active = false;
    }
}

pub fn select_next_validator(
    consensus_engine: &mut ConsensusEngine,
) -> Option<Address> {
    let now = Utc::now().timestamp_nanos_opt().ok_or("Couldn't fetch timestamp for UTC");
    let neurons_lock = consensus_engine.neurons.lock().map_err(|_| "Mutex poisoned").ok()?;

    let stake_weighted: Vec<(Address, u64)> = neurons_lock
        .values()
        .filter_map(|neuron| neuron.validator.clone().map(|v| (v, neuron.staked_amount)))
        .collect();

    if stake_weighted.is_empty() {
        return None;
    }

    let total_stake: u64 = stake_weighted.iter().map(|(_, stake)| stake).sum();
    let seed = now.unwrap().wrapping_add(total_stake as i64);
    let hash = Sha256::digest(&seed.to_be_bytes());
    let roll = u64::from_be_bytes(hash[0..8].try_into().unwrap()) % total_stake;

    let mut cumulative = 0;
    for (validator, stake) in stake_weighted {
        cumulative += stake;
        if roll < cumulative {
            return Some(validator);
        }
    }

    None
}