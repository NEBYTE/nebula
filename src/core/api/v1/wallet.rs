use std::sync::Arc;
use rocksdb::DB;

use crate::core::wallet::registrar::Wallet;
pub fn create_wallet(db: Arc<DB>) -> Wallet {
    Wallet::new(db)
}
