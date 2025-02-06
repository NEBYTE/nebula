use std::sync::{Arc};
use chrono::Utc;
use ed25519_dalek::SigningKey;
use crate::core::staking::staking_module::StakingModule;

pub fn stake(
    staking_module: &mut StakingModule,
    caller: &SigningKey,
    neuron_id: u64,
    amount: u64
) -> Result<(), String> {
    let mut neurons = staking_module.neurons.lock().unwrap();
    let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

    if neuron.private_address != Arc::new(caller.clone()) {
        return Err("Caller is not the owner of this neuron".to_string());
    }

    neuron.staked = true;
    neuron.staked_amount += amount;

    Ok(())
}

pub fn unstake(
    staking_module: &mut StakingModule,
    caller: &SigningKey,
    neuron_id: u64,
    amount: u64
) -> Result<(), String> {
    let mut neurons = staking_module.neurons.lock().unwrap();
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

