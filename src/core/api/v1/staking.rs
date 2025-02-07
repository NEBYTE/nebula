use crate::core::staking::staking_module::StakingModule;
use ed25519_dalek::SigningKey;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use crate::core::consensus::model::ConsensusEngine;

pub fn stake_tokens(
    canister: &mut Canister,
    consensus_engine: &mut ConsensusEngine,
    staking_module: &mut StakingModule,
    signing_key: &SigningKey,
    neuron_id: u64,
    amount: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Stake {
        staking_module,
        consensus_engine,
        signing_key,
        neuron_id,
        amount,
    })
}

pub fn unstake_tokens(
    canister: &mut Canister,
    consensus_engine: &mut ConsensusEngine,
    staking_module: &mut StakingModule,
    signing_key: &SigningKey,
    neuron_id: u64,
    amount: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Unstake {
        staking_module,
        consensus_engine,
        signing_key,
        neuron_id,
        amount,
    })
}
