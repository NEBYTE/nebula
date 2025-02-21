use crate::core::types::Transaction;
use crate::core::crypto::{verify_data};
use ed25519_dalek::VerifyingKey;
use bincode;
use hex;
use crate::core::consensus::{crypto_hash};
use crate::core::consensus::model::{Account, ConsensusEngine};

pub fn add_transaction(
    consensus_engine: &mut ConsensusEngine,
    tx: Transaction,
) -> Result<(), String> {
    {
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

        let mut ledger = consensus_engine.ledger.lock();

        let sender_account = match ledger.get(&tx.from) {
            Some(account) => account.clone(),
            None => {
                let new_account = Account {
                    address: tx.from.clone(),
                    public_key: sender_pubkey,
                    balance: 0,
                };
                ledger.insert(tx.from.clone(), new_account.clone());
                new_account
            }
        };

        if sender_account.balance < tx.amount {
            return Err(format!(
                "Insufficient funds: Sender balance is {} but transaction amount is {}",
                sender_account.balance, tx.amount
            ));
        }

        let receiver_account = match ledger.get(&tx.to) {
            Some(account) => account.clone(),
            None => return Err("Invalid transaction receiver account.".to_string()),
        };

        let updated_sender = Account {
            balance: sender_account.balance - tx.amount,
            ..sender_account.clone()
        };
        ledger.insert(tx.from.clone(), updated_sender);

        let updated_receiver = Account {
            balance: receiver_account.balance + tx.amount,
            ..receiver_account.clone()
        };
        ledger.insert(tx.to.clone(), updated_receiver);

        let mut mempool_lock = consensus_engine.mempool.lock();
        mempool_lock.push(tx);
    }

    consensus_engine.persist_state();
    Ok(())
}

pub fn cancel_transaction(consensus_engine: &mut ConsensusEngine, tx_hash: String) -> Result<(), String> {
    let mut mempool_lock = consensus_engine.mempool.lock();
    let pos = mempool_lock.iter().position(|tx| tx.hash == tx_hash);

    if let Some(index) = pos {
        mempool_lock.remove(index);
        drop(mempool_lock);

        consensus_engine.persist_state();
        Ok(())
    } else {
        Err("Transaction not found in mempool".to_string())
    }
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
