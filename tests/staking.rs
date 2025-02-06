use nebula::core::staking::staking_module::StakingModule;
use nebula::core::staking::staking_handler::{stake};
use nebula::core::types::Neuron;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[test]
fn test_stake() {
    let neurons = Arc::new(Mutex::new(HashMap::new()));
    let mut staking_module = StakingModule::new(neurons.clone());

    let signing_key = SigningKey::generate(&mut OsRng);
    let neuron_id = 1;

    neurons.lock().unwrap().insert(neuron_id, Neuron {
        name: "TestNeuron".to_string(),
        id: neuron_id,
        private_address: Arc::new(signing_key.clone()),
        staked: false,
        staked_amount: 0,
        unlock_date: chrono::Utc::now().date_naive(),
        state: nebula::core::types::NeuronStatus::NotDissolving,
        visibility: true,
        address: "test".to_string(),
        age: chrono::Utc::now().date_naive(),
        voting_power: 0,
        maturity: 0,
        bonus_multiplier: 1.0,
        date_created: chrono::Utc::now(),
        dissolve_delay_bonus: 0,
        age_bonus: 0,
        total_bonus: 0,
        is_genesis: false,
        is_known_neuron: false,
        validator: None,
    });

    let result = stake(&mut staking_module, &signing_key, neuron_id, 50);
    assert!(result.is_ok());
    assert_eq!(neurons.lock().unwrap().get(&neuron_id).unwrap().staked_amount, 50);
}
