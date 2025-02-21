use std::sync::{Arc};
use chrono::Utc;
use ed25519_dalek::SigningKey;
use crate::core::consensus::model::{ConsensusEngine};
use crate::core::nervous::NervousSystem;
use crate::core::staking::staking_module::StakingModule;

pub fn stake(
    nervous_system: &mut NervousSystem,
    staking_module: &mut StakingModule,
    consensus_engine: &mut ConsensusEngine,
    caller: &SigningKey,
    neuron_id: u64,
    amount: u64
) -> Result<(), String> {
    {
        let mut neurons = staking_module.neurons.lock();
        let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

        if neuron.private_address != Arc::new(caller.clone()) {
            return Err("Caller is not the owner of this neuron".to_string());
        }

        let mut ledger = consensus_engine.ledger.lock();

        let staker_account = ledger
            .get_mut(&neuron.address)
            .ok_or_else(|| "Staker account not found in ledger".to_string())?;

        if staker_account.balance < amount {
            return Err("Staker account balance exceeded".to_string());
        }

        staker_account.balance -= amount;
        neuron.staked = true;
        neuron.staked_amount += amount;
    }

    nervous_system.persist_neurons();
    consensus_engine.persist_state();
    Ok(())
}

pub fn unstake(
    nervous_system: &mut NervousSystem,
    staking_module: &mut StakingModule,
    consensus_engine: &mut ConsensusEngine,
    caller: &SigningKey,
    neuron_id: u64,
    amount: u64
) -> Result<(), String> {
    {
        let mut neurons = staking_module.neurons.lock();
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

        let mut ledger = consensus_engine.ledger.lock();
        let staker_account = ledger
            .get_mut(&neuron.address)
            .ok_or_else(|| "Staker account not found in ledger".to_string())?;
        staker_account.balance += amount;
    }

    nervous_system.persist_neurons();
    consensus_engine.persist_state();
    Ok(())
}

