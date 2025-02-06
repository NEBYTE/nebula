use crate::core::types::{Address};
use crate::core::consensus::ConsensusEngine;

pub fn delegate_stake(
    consensus_engine: &mut ConsensusEngine,
    neuron_id: u64,
    validator: Address
) -> Result<(), String> {
    let mut neurons = consensus_engine.neurons.lock().unwrap();

    let validator_list: Vec<Address> = neurons.values()
        .filter_map(|n| n.validator.clone())
        .collect();

    let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

    if !validator_list.contains(&validator) {
        return Err("Validator not found or inactive".into());
    }

    neuron.validator = Some(validator);
    Ok(())
}
