use std::sync::{Arc};
use rocksdb::DB;
use serde::{Serialize, Deserialize};
use bincode;
use ed25519_dalek::{SigningKey, VerifyingKey};
use crate::core::types::Address;

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub signing_key: SigningKey,
    pub public_key: VerifyingKey,
    pub address: Address,
}

impl Wallet {
    pub fn new(db: Arc<DB>) -> Self {
        if let Some(wallet) = Self::load_state(&db) {
            return wallet;
        }

        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let public_key = VerifyingKey::from(&signing_key);
        let address = hex::encode(public_key.to_bytes());

        let wallet = Wallet { signing_key, public_key, address };
        wallet.persist_state(&db);

        wallet
    }

    pub fn persist_state(&self, db: &Arc<DB>) {
        match bincode::serialize(self) {
            Ok(serialized) => {
                db.put(b"wallet_wallet", serialized).expect("Failed to store wallet in database.");
            }
            Err(e) => {
                eprintln!("Wallet serialization failed: {}", e);
            }
        }
    }

    pub fn load_state(db: &Arc<DB>) -> Option<Self> {
        if let Ok(Some(data)) = db.get(b"wallet_wallet") {
            match bincode::deserialize::<Wallet>(&data) {
                Ok(wallet) => Some(wallet),
                Err(e) => {
                    eprintln!("Failed to deserialize wallet: {}", e);
                    None
                }
            }
        } else {
            None
        }
    }
}

