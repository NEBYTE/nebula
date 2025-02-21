use std::sync::Arc;
use crate::core::types::{Address, MutexWrapper};
use sha2::{Digest, Sha256};
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::core::consensus::model::ConsensusEngine;
use crate::core::nervous::NervousSystem;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidatorInfo {
    pub address: Address,
    pub neuron_id: u64,
    pub active: bool,
}
pub fn slash(
    consensus_engine: &mut ConsensusEngine,
    validator: Address,
    amount: u64,
) {
    {
        let penalty = amount * 2;
        let mut neurons = consensus_engine.neurons.lock();

        let mut validators_lock = consensus_engine.validators.lock();

        for neuron in neurons.values_mut() {
            if neuron.validator == Some(validator.clone()) {
                if neuron.staked_amount > penalty {
                    neuron.staked_amount -= penalty;
                } else {
                    neuron.staked_amount = 0;
                }
            }
        }

        if let Some(v) = validators_lock.iter_mut().find(|v| v.address == validator) {
            v.active = false;
        }
    }

    consensus_engine.persist_state();
}

pub fn build_validator(nervous_system: &mut NervousSystem, neuron_id: u64) -> Result<ValidatorInfo, String> {
    let neurons_lock = nervous_system.neurons.lock();
    let neuron = neurons_lock.get(&neuron_id).ok_or("Neuron not exist")?;

    let validator = ValidatorInfo {
        address: neuron.address.clone(),
        neuron_id,
        active: false,
    };

    drop(neurons_lock);

    Ok(validator)
}

pub fn wrap_validator(validator_info: ValidatorInfo) -> Arc<MutexWrapper<Vec<ValidatorInfo>>> {
   Arc::new(MutexWrapper::new(vec![validator_info]))
}

pub fn register_validator(consensus_engine: &mut ConsensusEngine, neuron_id: u64) -> Result<(), String> {
    {
        let neurons_lock = consensus_engine.neurons.lock();
        let neuron = neurons_lock.get(&neuron_id).ok_or("Neuron not exist")?;

        let mut validators_lock = consensus_engine.validators.lock();

        if validators_lock.iter().any(|v| v.address == neuron.address) {
            return Err("Neuron already exists".into());
        }

        if neuron.staked_amount < 100 {
            return Err("Neuron does not have enough stake".into());
        }

        validators_lock.push(ValidatorInfo {
            address: neuron.address.clone(),
            neuron_id,
            active: true,
        });
    }

    consensus_engine.persist_state();
    Ok(())
}
pub fn select_next_validator(
    consensus_engine: &mut ConsensusEngine,
) -> Option<Address> {
    let now = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let neurons_lock = consensus_engine.neurons.lock();
    let validators_lock = consensus_engine.validators.lock();

    let stake_weighted: Vec<(Address, u64)> = neurons_lock
        .values()
        .filter_map(|neuron| {
            neuron.validator.as_ref().and_then(|validator_address| {
                validators_lock.iter().find(|v| v.address == *validator_address)
                    .map(|validator| (validator.address.clone(), neuron.staked_amount))
            })
        })
        .collect();


    if stake_weighted.is_empty() {
        return None;
    }

    let total_stake: u64 = stake_weighted.iter().map(|(_, stake)| stake).sum();
    let seed = now.wrapping_add(rand::thread_rng().gen_range(0..total_stake as i64));
    let hash = Sha256::digest(&seed.to_be_bytes());
    let roll = u64::from_be_bytes(hash[0..8].try_into().unwrap()) % total_stake;

    let mut cumulative = 0;
    for (validator, stake) in stake_weighted {
        cumulative += stake;
        if roll < cumulative {
            drop(validators_lock);
            drop(neurons_lock);

            consensus_engine.persist_state();
            return Some(validator);
        }
    }

    None
}