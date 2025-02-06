use nebula::core::governance::governance::Governance;
use nebula::core::governance::proposal_handler::propose;
use nebula::core::governance::voting::{vote, finalize};
use nebula::core::types::{Neuron};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[test]
fn test_governance_proposal_and_voting() {
    let neurons = Arc::new(Mutex::new(HashMap::new()));
    let governance = Governance::new(neurons.clone());

    let signing_key = SigningKey::generate(&mut OsRng);
    let neuron_id = 1;

    neurons.lock().unwrap().insert(neuron_id, Neuron {
        name: "TestNeuron".to_string(),
        id: neuron_id,
        private_address: Arc::new(signing_key.clone()),
        staked: true,
        staked_amount: 100,
        unlock_date: chrono::Utc::now().date_naive(),
        state: nebula::core::types::NeuronStatus::NotDissolving,
        visibility: true,
        address: "test".to_string(),
        age: chrono::Utc::now().date_naive(),
        voting_power: 100,
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

    let proposal_id = propose(&governance, "Test Proposal".to_string(), &signing_key, neuron_id).unwrap();
    assert!(proposal_id > 0);

    let vote_result = vote(&governance, &signing_key, neuron_id, proposal_id, true, 10);
    assert!(vote_result.is_ok());

    let finalize_result = finalize(&governance, proposal_id);
    assert!(finalize_result.is_ok());
}
