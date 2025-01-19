/// API for Nebula.
/// Exposes wallet creation and building transactions.
use crate::types::{Transaction, Address};
use ed25519_dalek::{SigningKey};
use rand::rngs::OsRng;

pub fn create_wallet() -> Address {
    let keypair = SigningKey::generate(&mut OsRng);
    keypair.verifying_key().to_bytes()
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
