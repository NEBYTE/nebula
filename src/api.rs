use crate::types::{Transaction, Address, Block};
use crate::crypto::{sign_transaction};
use crate::consensus::ConsensusEngine;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sled::Db;

pub fn create_wallet() -> (SigningKey, Address) {
    let keypair = SigningKey::generate(&mut OsRng);
    let address = keypair.verifying_key().to_bytes();
    (keypair, address)
}

pub fn build_transaction(from: Address, to: Address, amount: u64, nonce: u64) -> Transaction {
    Transaction {
        from,
        to,
        amount,
        nonce,
        signature: vec![],
    }
}

pub fn sign_and_build_transaction(signing_key: &SigningKey, tx: &mut Transaction) {
    sign_transaction(tx, signing_key);
}

pub fn submit_transaction(db: &Db, tx: &Transaction) -> Result<(), String> {
    let serialized = bincode::serialize(tx).map_err(|e| format!("Serialization failed: {}", e))?;
    db.insert(tx.nonce.to_be_bytes(), serialized)
        .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

pub fn get_transaction(db: &Db, nonce: u64) -> Option<Transaction> {
    let key = nonce.to_be_bytes();
    db.get(key).ok().flatten().and_then(|raw| {
        bincode::deserialize(&raw).ok()
    })
}

pub fn get_current_validator(consensus: &ConsensusEngine) -> Option<Address> {
    consensus.select_next_validator()
}

pub fn get_latest_block(db: &Db) -> Option<Block> {
    db.get(b"latest_block").ok().flatten().and_then(|raw| {
        bincode::deserialize(&raw).ok()
    })
}
