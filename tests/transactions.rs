use nebula::core::api::v1::transaction::{build_transaction, finalize_transaction, submit_transaction};
use nebula::core::api::v1::wallet::create_wallet;
use nebula::core::consensus::model::ConsensusEngine;
use nebula::core::types::TransactionType;
use nebula::core::canister::canister::Canister;
use nebula::core::nervous::NervousSystem;

#[test]
fn test_full_transaction_flow() {
    let nervous_system = NervousSystem::new();

    let validators = vec![];
    let mut consensus_engine = ConsensusEngine::new(validators, nervous_system.neurons.clone());

    let (signing_key, public_key, address) = create_wallet();
    let (recipient_signing_key, recipient_public_key, recipient_address) = create_wallet();

    let amount = 100;

    let canister_id = "test_canister".to_string();
    let mut canister = Canister::new(canister_id, address.clone());

    consensus_engine.init_ledger(address.clone(), public_key, amount);
    consensus_engine.init_ledger(recipient_address.clone(), recipient_public_key, amount); // Making sure recipient wallet exists in ledger

    let mut tx = build_transaction(
        &mut consensus_engine,
        address.clone(),
        recipient_address.clone(),
        amount,
        0,
        0,
        TransactionType::Transfer,
    );

    finalize_transaction(&mut tx, &signing_key).expect("Finalization failed");

    submit_transaction(&mut canister, &mut consensus_engine, tx)
        .expect("Transaction submission failed");

    assert_eq!(
        consensus_engine.mempool.len(),
        1,
        "Mempool should contain 1 transaction"
    );
}
