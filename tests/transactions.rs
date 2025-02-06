use nebula::core::api::v1::*;
use nebula::core::consensus::*;
use nebula::core::types::TransactionType;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[test]
fn test_full_transaction_flow() {
    let validators = vec![];
    let neurons = Arc::new(Mutex::new(HashMap::new()));
    let mut consensus_engine = ConsensusEngine::new(validators, neurons.clone());

    let (signing_key, _verifying_key, address) = create_wallet();
    let recipient = "recipient_address".to_string();
    let amount = 100;

    let mut tx = build_transaction(
        &mut consensus_engine,
        address.clone(),
        recipient.clone(),
        amount,
        0,
        0,
        TransactionType::Transfer
    );

    finalize_transaction(&mut tx, &signing_key).expect("Finalization failed");
    submit_transaction(&mut consensus_engine, tx).expect("Transaction submission failed");

    assert_eq!(consensus_engine.mempool.len(), 1);
}
