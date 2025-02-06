use crate::core::types::Transaction;
use crate::core::crypto::{verify_data};
use ed25519_dalek::VerifyingKey;
use bincode;
use hex;
use crate::core::consensus::{crypto_hash, ConsensusEngine};

pub fn add_transaction(
    consensus_engine: &mut ConsensusEngine,
    tx: Transaction
) -> Result<(), String> {
    let expected_hash = compute_transaction_hash(&tx)?;

    if tx.hash != expected_hash {
        return Err(format!(
            "Invalid transaction hash: Expected {}, got {}",
            expected_hash, tx.hash
        ));
    }

    let sender_pubkey_bytes = hex::decode(&tx.from)
        .map_err(|_| format!("Invalid sender address format: {}", &tx.from))?;

    if sender_pubkey_bytes.len() != 32 {
        return Err(format!(
            "Invalid public key length: Expected 32 bytes, got {}",
            sender_pubkey_bytes.len()
        ));
    }

    let sender_pubkey = VerifyingKey::from_bytes(&sender_pubkey_bytes.try_into().unwrap())
        .map_err(|_| "Invalid sender public key: Failed to create VerifyingKey")?;

    let signature_copy = tx.signature.clone();

    let mut tx_clone = tx.clone();
    tx_clone.signature.clear();
    tx_clone.hash.clear();

    let serialized_tx = bincode::serialize(&tx_clone)
        .map_err(|e| format!("Verification Serialization Error: {}", e))?;

    if signature_copy.is_empty() {
        return Err("Invalid transaction signature: Signature is missing.".to_string());
    }

    if !verify_data(&sender_pubkey, &serialized_tx, &signature_copy) {
        return Err("Invalid transaction signature: Signature does not match.".to_string());
    }

    consensus_engine.mempool.push(tx);
    Ok(())
}

pub fn compute_transaction_hash(
    tx: &Transaction
) -> Result<String, String> {
    let mut tx_clone = tx.clone();
    tx_clone.hash.clear();
    tx_clone.signature.clear();

    let bytes = bincode::serialize(&tx_clone).map_err(|e| e.to_string())?;
    let hash_bytes = crypto_hash(&bytes);
    Ok(hex::encode(hash_bytes))
}
