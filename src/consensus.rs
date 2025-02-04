use crate::types::{Block, BlockHeader, Transaction, Address, Neuron};
use crate::crypto::{sign_data, verify_data};
use ed25519_dalek::{SigningKey, VerifyingKey};
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hex;

#[derive(Clone)]
pub struct ValidatorInfo {
    pub address: Address,
    pub active: bool,
}

pub struct ConsensusEngine {
    pub validators: Vec<ValidatorInfo>,
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
    pub mempool: Vec<Transaction>,
    pub chain: Vec<Block>,
}

impl ConsensusEngine {
    pub fn new(validators: Vec<ValidatorInfo>, neurons: Arc<Mutex<HashMap<u64, Neuron>>>) -> Self {
        Self {
            validators,
            neurons,
            mempool: Vec::new(),
            chain: Vec::new(),
        }
    }
    pub fn select_next_validator(&self) -> Option<Address> {
        let mut stake_map = HashMap::new();
        let neurons = self.neurons.lock().unwrap();
        for neuron in neurons.values() {
            if let Some(v) = &neuron.validator {
                *stake_map.entry(v).or_insert(0) += neuron.staked_amount;
            }
        }

        stake_map.iter().max_by_key(|(_, stake)| *stake).map(|(a, _)| *a).cloned()
    }
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        let computed = compute_transaction_hash(&tx)?;
        if tx.hash != computed {
            return Err("Invalid transaction hash".into());
        }
        self.mempool.push(tx);

        Ok(())
    }
    pub fn produce_block(&mut self, signing_key: &SigningKey) -> Result<Block, String> {
        let transactions = self.mempool.drain(..).collect::<Vec<_>>();
        let merkle_root = compute_merkle_root(&transactions);

        let verifying_key = signing_key.verifying_key();
        let parent_hash = self.chain.last().map(|blk| hash_block(blk)).unwrap_or([0u8; 32]);

        let mut header = BlockHeader {
            parent_hash,
            merkle_root,
            timestamp: Utc::now().timestamp() as u64,
            validator: hex::encode(verifying_key.to_bytes()),
            signature: vec![],
        };

        let signable = serialize_header_for_signing(&header)?;
        header.signature = sign_data(signing_key, &signable);

        let block = Block { header, transactions };
        self.chain.push(block.clone());

        Ok(block)
    }
    pub fn validate_block(&self, block: &Block) -> Result<(), String> {
        let pubkey_bytes = hex::decode(&block.header.validator)
            .map_err(|e| format!("Invalid hex address: {}", e))?;

        let pubkey_array: [u8; 32] = pubkey_bytes
            .try_into()
            .map_err(|_| "Invalid length: Expected 32 bytes".to_string())?;

        let pubkey = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|e| format!("Failed to create VerifyingKey: {}", e))?;

        if !self.validators.iter().any(|v| v.address == block.header.validator && v.active) {
            return Err("Block validator is not active".into());
        }

        let signable = serialize_header_for_signing(&block.header)?;
        if !verify_data(&pubkey, &signable, &block.header.signature) {
            return Err("Invalid block signature".into());
        }

        let computed_merkle_root = compute_merkle_root(&block.transactions);
        if block.header.merkle_root != computed_merkle_root {
            return Err("Merkle root mismatch".into());
        }

        let now = Utc::now().timestamp() as u64;
        if block.header.timestamp > now + 600 {
            return Err("Block timestamp is too far in the future".into());
        }

        Ok(())
    }
    pub fn delegate_stake(&mut self, neuron_id: u64, validator: Address) -> Result<(), String> {
        let mut neurons = self.neurons.lock().unwrap();
        let neuron = neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;
        if !self.validators.iter().any(|v| v.address == validator && v.active) {
            return Err("Validator not found or inactive".into());
        }

        neuron.validator = Some(validator);
        Ok(())
    }
    pub fn slash(&mut self, validator: Address, amount: u64) {
        let penalty = amount * 2;
        let mut neurons = self.neurons.lock().unwrap();
        for neuron in neurons.values_mut() {
            if neuron.validator == Some(validator.clone()) {
                if neuron.staked_amount > penalty {
                    neuron.staked_amount -= penalty;
                } else {
                    neuron.staked_amount = 0;
                }
            }
        }

        if let Some(v) = self.validators.iter_mut().find(|v| v.address == validator) {
            v.active = false;
        }
    }
}

pub fn crypto_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn compute_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
    if transactions.is_empty() {
        return [0; 32];
    }

    let mut hashes: Vec<[u8; 32]> = transactions
        .iter()
        .map(|tx| {
            let mut tx_clone = tx.clone();
            tx_clone.hash.clear();
            let bytes = bincode::serialize(&tx_clone).unwrap_or_default();
            crypto_hash(&bytes)
        })
        .collect();

    while hashes.len() > 1 {
        let mut new_hashes = Vec::new();
        for chunk in hashes.chunks(2) {
            let mut hasher = Sha256::new();
            hasher.update(&chunk[0]);
            if chunk.len() == 2 {
                hasher.update(&chunk[1]);
            }
            new_hashes.push(hasher.finalize().into());
        }
        hashes = new_hashes;
    }

    hashes[0]
}

pub fn hash_block(block: &Block) -> [u8; 32] {
    let signable = serialize_header_for_signing(&block.header).unwrap_or_default();
    crypto_hash(&signable)
}

pub fn serialize_header_for_signing(header: &BlockHeader) -> Result<Vec<u8>, String> {
    let mut h = header.clone();
    h.signature.clear();
    bincode::serialize(&h).map_err(|e| e.to_string())
}

pub fn compute_transaction_hash(tx: &Transaction) -> Result<String, String> {
    let mut tx_clone = tx.clone();
    tx_clone.hash.clear();

    let bytes = bincode::serialize(&tx_clone).map_err(|e| e.to_string())?;
    let hash_bytes = crypto_hash(&bytes);
    Ok(hex::encode(hash_bytes))
}
