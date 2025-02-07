use crate::core::staking::staking_module::StakingModule;
use ed25519_dalek::SigningKey;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};

pub fn stake_tokens(
    canister: &mut Canister,
    staking_module: &mut StakingModule,
    signing_key: &SigningKey,
    neuron_id: u64,
    amount: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Stake {
        staking_module,
        signing_key,
        neuron_id,
        amount,
    })
}

pub fn unstake_tokens(
    canister: &mut Canister,
    staking_module: &mut StakingModule,
    signing_key: &SigningKey,
    neuron_id: u64,
    amount: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Unstake {
        staking_module,
        signing_key,
        neuron_id,
        amount,
    })
}
