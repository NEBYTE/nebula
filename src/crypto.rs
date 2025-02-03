use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Verifier, Signer};
use crate::types::Transaction;

pub fn sign_data(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
    let signature: Signature = signing_key.sign(message);
    signature.to_bytes().to_vec()
}

pub fn verify_data(verifying_key: &VerifyingKey, message: &[u8], signature: &[u8]) -> bool {
    if let Ok(sig) = Signature::try_from(signature) {
        verifying_key.verify(message, &sig).is_ok()
    } else {
        false
    }
}

pub fn sign_transaction(tx: &mut Transaction, signing_key: &SigningKey) {
    let raw = bincode::serialize(&*tx).unwrap();
    tx.signature = sign_data(signing_key, &raw);
}
