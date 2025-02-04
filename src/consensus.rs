use crate::types::{Block, BlockHeader, Transaction, Address, Neuron};
use crate::crypto::{sign_data, verify_data};
use sled::Db;
use ed25519_dalek::{SigningKey, VerifyingKey};
use bincode::serialize;
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
#[derive(Clone)]
pub struct ValidatorInfo {
    pub address: Address,
    pub active: bool,
}

pub struct ConsensusEngine {
    pub validators: Vec<ValidatorInfo>,
    pub neurons: HashMap<u64, Neuron>,
    pub db: Db,
}

impl ConsensusEngine {
    pub fn new(validators: Vec<ValidatorInfo>, db: Db) -> Self {
        Self {
            validators,
            neurons: HashMap::new(),
            db,
        }
    }

    pub fn select_next_validator(&self) -> Option<Address> {
        let mut stake_map: HashMap<Address, u64> = HashMap::new();

        for neuron in self.neurons.values() {
            if let Some(validator) = neuron.validator {
                *stake_map.entry(validator).or_insert(0) += neuron.staked_amount;
            }
        }

        stake_map.iter()
            .max_by_key(|(_, stake)| *stake)
            .map(|(addr, _)| *addr)
    }

    pub fn produce_block(&self, signing_key: &SigningKey, transactions: Vec<Transaction>, parent_hash: [u8; 32]) -> Block {
        let merkle_root = compute_merkle_root(&transactions);
        let verifying_key = signing_key.verifying_key();

        let header = BlockHeader {
            parent_hash,
            merkle_root,
            timestamp: Utc::now().timestamp() as u64,
            validator: verifying_key.to_bytes(),
            signature: vec![],
        };

        let encoded = serialize(&header).unwrap();
        let signature = sign_data(signing_key, &encoded);

        Block {
            header: BlockHeader { signature, ..header },
            transactions,
        }
    }

    pub fn validate_block(&self, block: &Block) -> Result<(), String> {
        let encoded = serialize(&block.header).map_err(|e| e.to_string())?;
        let pubkey = VerifyingKey::from_bytes(&block.header.validator)
            .map_err(|_| "Invalid pubkey".to_owned())?;

        let valid_val = self.validators.iter()
            .any(|v| v.address == block.header.validator && v.active);
        if !valid_val {
            return Err("Invalid block validator".into());
        }

        if !verify_data(&pubkey, &encoded, &block.header.signature) {
            return Err("Invalid block signature".into());
        }

        let computed_merkle_root = compute_merkle_root(&block.transactions);
        if block.header.merkle_root != computed_merkle_root {
            return Err("Merkle root mismatch".into());
        }

        let now = Utc::now().timestamp() as u64;
        if block.header.timestamp > now + 600 {
            return Err("Block timestamp is before now".into());
        }

        Ok(())
    }

    pub fn delegate_stake(&mut self, neuron_id: u64, validator: Address) -> Result<(), String> {
        let neuron = self.neurons.get_mut(&neuron_id).ok_or("Neuron not found")?;

        if !self.validators.iter().any(|v| v.address == validator && v.active) {
            return Err("Validator not found or inactive".to_string());
        }

        neuron.validator = Some(validator);
        Ok(())
    }

    pub fn slash(&mut self, validator: Address, amount: u64) {
        let penalty = amount * 2;

        for neuron in self.neurons.values_mut() {
            if neuron.validator == Some(validator) {
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

pub fn compute_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
    if transactions.is_empty() {
        return [0; 32];
    }

    let mut hashes: Vec<[u8; 32]> = transactions.iter()
        .map(|tx| crypto_hash(&serialize(tx).unwrap()))
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

pub fn crypto_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}