use crate::core::staking::staking_module::StakingModule;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use crate::core::consensus::model::ConsensusEngine;
use crate::core::nervous::NervousSystem;

use ed25519_dalek::SigningKey;

pub fn stake_tokens(
    canister: &mut Canister,
    nervous_system: &mut NervousSystem,
    consensus_engine: &mut ConsensusEngine,
    staking_module: &mut StakingModule,
    signing_key: &SigningKey,
    neuron_id: u64,
    amount: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Stake {
        nervous_system,
        staking_module,
        consensus_engine,
        signing_key,
        neuron_id,
        amount,
    })
}

pub fn unstake_tokens(
    canister: &mut Canister,
    nervous_system: &mut NervousSystem,
    consensus_engine: &mut ConsensusEngine,
    staking_module: &mut StakingModule,
    signing_key: &SigningKey,
    neuron_id: u64,
    amount: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Unstake {
        nervous_system,
        staking_module,
        consensus_engine,
        signing_key,
        neuron_id,
        amount,
    })
}
