/// Consensus for Nebula.
/// Selects validator, produces blocks, validates blocks.

use crate::types::{Block, BlockHeader, Transaction, Address};
use crate::crypto::{sign_data, verify_data};
use sled::Db;
use ed25519_dalek::{SigningKey, VerifyingKey};
use bincode::serialize;
#[derive(Clone)]
pub struct ValidatorInfo {
    pub address: Address,
    pub stake: u64,
    pub active: bool,
}

pub struct ConsensusEngine {
    pub validators: Vec<ValidatorInfo>,
    pub db: Db,
}

impl ConsensusEngine {
    pub fn new(validators: Vec<ValidatorInfo>, db: Db) -> Self {
        Self { validators, db }
    }

    pub fn select_next_validator(&self) -> Option<Address> {
        self.validators.first().map(|v| v.address)
    }

    pub fn produce_block(&self, signing_key: &SigningKey, transactions: Vec<Transaction>) -> Block {
        let tx_data = serialize(&transactions).unwrap();
        let merkle_root = crypto_hash(&tx_data);

        let verifying_key = signing_key.verifying_key();
        let header = BlockHeader {
            parent_hash: crypto_hash(b"placeholder-parent"),
            merkle_root,
            timestamp: 0,
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

        Ok(())
    }

    pub fn slash(&mut self, address: Address, amount: u64) {
        if let Some(v) = self.validators.iter_mut().find(|x| x.address == address) {
            if v.stake > amount {
                v.stake -= amount;
            } else {
                v.stake = 0;
                v.active = false;
            }
        }
    }
}

pub fn crypto_hash(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, &b) in data.iter().enumerate().take(32) {
        out[i] = b.wrapping_mul(13);
    }
    out
}
