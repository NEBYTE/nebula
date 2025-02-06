use crate::core::types::{Block, BlockHeader, Transaction};
use crate::core::crypto::{sign_data, verify_data};
use crate::core::consensus::{crypto_hash, ConsensusEngine};

use sha2::{Digest, Sha256};
use chrono::Utc;
use ed25519_dalek::{SigningKey, VerifyingKey};
use bincode;
use hex;

pub fn produce_block(
    consensus_engine: &mut ConsensusEngine,
    signing_key: &SigningKey
) -> Result<Block, String> {
    let transactions = consensus_engine.mempool.drain(..).collect::<Vec<_>>();
    let merkle_root = compute_merkle_root(&transactions);

    let verifying_key = signing_key.verifying_key();
    let parent_hash = consensus_engine.chain.last().map(|blk| hash_block(blk)).unwrap_or([0u8; 32]);

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
    consensus_engine.chain.push(block.clone());

    Ok(block)
}

pub fn validate_block(
    consensus_engine: &mut ConsensusEngine,
    block: &Block,
) -> Result<(), String> {
    let pubkey_bytes = hex::decode(&block.header.validator)
    .map_err(|e| format!("Invalid hex address: {}", e))?;

    let pubkey_array: [u8; 32] = pubkey_bytes
        .try_into()
        .map_err(|_| "Invalid length: Expected 32 bytes".to_string())?;

    let pubkey = VerifyingKey::from_bytes(&pubkey_array)
        .map_err(|e| format!("Failed to create VerifyingKey: {}", e))?;

    if !consensus_engine.validators.iter().any(|v| v.address == block.header.validator && v.active) {
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

pub fn compute_merkle_root(
    transactions: &[Transaction]
) -> [u8; 32] {
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

pub fn hash_block(
    block: &Block
) -> [u8; 32] {
    let signable = serialize_header_for_signing(&block.header).unwrap_or_default();
    crypto_hash(&signable)
}

pub fn serialize_header_for_signing(
    header: &BlockHeader
) -> Result<Vec<u8>, String> {
    let mut h = header.clone();
    h.signature.clear();
    bincode::serialize(&h).map_err(|e| e.to_string())
}
