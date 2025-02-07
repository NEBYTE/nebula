use ed25519_dalek::SigningKey;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use crate::core::consensus::model::ConsensusEngine;

pub fn produce_block(
    canister: &mut Canister,
    consensus_engine: &mut ConsensusEngine,
    signing_key: &SigningKey,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::ProduceBlock {
        consensus_engine,
        signing_key,
    })
}

pub fn select_next_validator(
    canister: &mut Canister,
    consensus_engine: &mut ConsensusEngine,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::SelectValidator {
        consensus_engine,
    })
}