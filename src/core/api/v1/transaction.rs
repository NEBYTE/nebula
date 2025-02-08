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

    let chain_guard = consensus.chain.lock().unwrap();
    let total_chain_txs: usize = chain_guard.iter().map(|block| block.transactions.len()).sum();

    let mempool_guard = consensus.mempool.lock().unwrap();
    let mempool_len = mempool_guard.len();

    let index = total_chain_txs as u32 + mempool_len as u32;

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

pub fn cancel_transaction(canister: &mut Canister, consensus_engine: &mut ConsensusEngine, tx_hash: String) -> Result<String, String> {
    canister.execute_function(CanisterFunctionPayload::CancelTransfer {
        consensus_engine,
        tx_hash,
    })
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
