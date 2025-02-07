use crate::core::governance::Governance;
use ed25519_dalek::SigningKey;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};

pub fn propose(
    canister: &mut Canister,
    governance: &mut Governance,
    topic: String,
    signing_key: &SigningKey,
    neuron_id: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Propose {
        governance,
        topic,
        signing_key,
        neuron_id,
    })
}

pub fn vote(
    canister: &mut Canister,
    governance: &mut Governance,
    signing_key: &SigningKey,
    neuron_id: u64,
    proposal_id: u64,
    vote: bool,
    stake_amount: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Vote {
        governance,
        signing_key,
        neuron_id,
        proposal_id,
        vote,
        stake_amount,
    })
}
