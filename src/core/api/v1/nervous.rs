use crate::core::nervous::NervousSystem;
use ed25519_dalek::SigningKey;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};

pub fn create_neuron(
    canister: &mut Canister,
    nervous_system: &mut NervousSystem,
    signing_key: &SigningKey,
    name: String,
    dissolve_days: i64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::CreateNeuron {
        nervous_system,
        signing_key,
        name,
        dissolve_days,
    })
}

pub fn get_neuron(
    canister: &mut Canister,
    nervous_system: &mut NervousSystem,
    neuron_id: u64,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::GetNeuron {
        nervous_system,
        neuron_id,
    })
}

pub fn list_neurons(
    canister: &mut Canister,
    nervous_system: &mut NervousSystem,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::ListNeurons {
        nervous_system,
    })
}
