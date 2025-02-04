use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use std::convert::TryFrom;

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