use crate::types::{Transaction, TransactionType, TransactionStatus, Address, Block};
use crate::consensus::{ConsensusEngine, compute_transaction_hash};
use chrono::Utc;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use hex;

pub fn create_wallet() -> (SigningKey, VerifyingKey, Address) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    let address = hex::encode(signing_key.verifying_key().to_bytes()) as Address;
    (signing_key, verifying_key, address)
}

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
    }
}

pub fn finalize_transaction(tx: &mut Transaction) -> Result<(), String> {
    let hash = compute_transaction_hash(tx)?;
    tx.hash = hash;
    Ok(())
}

pub fn submit_transaction(consensus: &mut ConsensusEngine, tx: Transaction) -> Result<(), String> {
    consensus.add_transaction(tx)
}

pub fn get_current_validator(consensus: &ConsensusEngine) -> Option<Address> {
    consensus.select_next_validator()
}

pub fn get_latest_block(consensus: &ConsensusEngine) -> Option<Block> {
    consensus.chain.last().cloned()
}
