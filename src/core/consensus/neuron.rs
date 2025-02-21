use crate::core::consensus::model::ConsensusEngine;
use crate::core::types::Address;

pub fn delegate_stake(
    consensus_engine: &mut ConsensusEngine,
    neuron_id: u64,
    validator: Address
) -> Result<(), String> {
    {
        let mut neurons = consensus_engine.neurons.lock();

        let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

        let validators_lock = consensus_engine.validators.lock();

        let is_valid_validator = validators_lock
            .iter()
            .any(|v| v.address == validator);

        if !is_valid_validator {
            return Err("Validator not found or inactive".into());
        }

        neuron.validator = Some(validator.clone());
    }

    consensus_engine.persist_state();
    Ok(())
}
