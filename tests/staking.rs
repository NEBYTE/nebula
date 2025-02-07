use nebula::core::staking::staking_module::StakingModule;
use nebula::core::staking::staking_handler::stake;
use nebula::core::consensus::model::ConsensusEngine;
use nebula::core::consensus::ValidatorInfo;
use nebula::core::nervous::NervousSystem;
use nebula::core::api::v1::wallet::create_wallet;
use nebula::core::nervous::neuron_handler::create_neuron;

#[test]
fn test_stake() {
    let nervous_system = NervousSystem::new();

    let (signing_key, public_key, sender_address) = create_wallet();
    let validators = vec![ValidatorInfo {
        address: sender_address.clone(),
        active: true
    }];

    let mut consensus_engine = ConsensusEngine::new(validators, nervous_system.neurons.clone());
    consensus_engine
        .init_ledger(sender_address.clone(), public_key, 100)
        .expect("Failed to initialize sender ledger");

    let neuron_id = create_neuron(
        &nervous_system,
        &signing_key,
        "TestNeuron".to_string(),
        30,
    )
        .expect("Failed to create neuron");

    let mut staking_module = StakingModule::new(nervous_system.neurons.clone());

    let amount_to_stake = 50;
    let result = stake(
        &mut staking_module,
        &mut consensus_engine,
        &signing_key,
        neuron_id,
        amount_to_stake,
    );
    assert!(result.is_ok(), "Staking failed: {:?}", result.err());

    let neurons_lock = nervous_system.neurons.lock().unwrap();
    let updated_neuron = neurons_lock
        .get(&neuron_id)
        .expect("Neuron not found");
    assert_eq!(
        updated_neuron.staked_amount,
        amount_to_stake,
        "Stake amount incorrect"
    );

    let sender_balance = consensus_engine
        .get_balance(&sender_address);

    assert_eq!(
        sender_balance,
        100 - amount_to_stake,
        "Ledger balance incorrect after staking"
    );
}
