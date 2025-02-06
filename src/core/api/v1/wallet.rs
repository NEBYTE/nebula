use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use hex;
use crate::core::types::Address;

pub fn create_wallet() -> (SigningKey, VerifyingKey, Address) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    let address = hex::encode(verifying_key.to_bytes());

    (signing_key, verifying_key, address)
}
