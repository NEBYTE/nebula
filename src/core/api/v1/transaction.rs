use crate::core::types::{Transaction, TransactionType, TransactionStatus, Address};
use crate::core::consensus::{compute_transaction_hash};
use crate::core::crypto::sign_data;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use crate::core::consensus::model::ConsensusEngine;

use chrono::Utc;
use ed25519_dalek::SigningKey;

pub fn build_transaction(
    consensus: &mut ConsensusEngine,
    from: Address,
    to: Address,
    amount: u64,
    memo: u32,
    nrc_memo: u32,
    tx_type: TransactionType,
) -> Transaction {
    let fee = amount / 100;
    let index = consensus
        .chain
        .iter()
        .map(|chain| chain.transactions.len())
        .sum::<usize>() as u32
        + consensus.mempool.len() as u32;

    Transaction {
        hash: String::new(),
        r#type: tx_type,
        status: TransactionStatus::Pending,
        index,
        timestamp: Utc::now(),
        from,
        to,
        amount,
        fee,
        memo,
        nrc_memo,
        signature: vec![],
    }
}

pub fn finalize_transaction(tx: &mut Transaction, signing_key: &SigningKey) -> Result<(), String> {
    let mut tx_clone = tx.clone();
    tx_clone.hash.clear();
    tx_clone.signature.clear();

    tx.hash = compute_transaction_hash(&tx_clone)?;

    let serialized_tx = bincode::serialize(&tx_clone)
        .map_err(|e| format!("Serialization Error: {}", e))?;
    tx.signature = sign_data(signing_key, &serialized_tx);

    Ok(())
}

pub fn submit_transaction(
    canister: &mut Canister,
    consensus_engine: &mut ConsensusEngine,
    tx: Transaction,
) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::Transfer {
        consensus_engine,
        tx,
    })
}
