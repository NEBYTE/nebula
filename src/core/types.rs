#![allow(dead_code)]

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use parking_lot::Mutex;
use ed25519_dalek::SigningKey;
use rocksdb::DB;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error as DeError;
pub type Address = String;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum VotingStatus {
    Open,
    Pending,
    Terminated,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TransactionType {
    Transfer,
    Mint,
    Approve,
    Burn,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TransactionStatus {
    Completed,
    Failed,
    Pending,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum NeuronStatus {
    NotDissolving,
    Dissolving,
    Dissolved,
    Spawning,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Vote {
    None,
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct VotingNeuron {
    pub name: String,
    pub id: u64,
    pub vote: Vote,
    pub private_address: Arc<SigningKey>,
}

fn serialize_signing_key<S>(key: &Arc<SigningKey>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(&key.to_bytes())
}

fn deserialize_signing_key<'de, D>(deserializer: D) -> Result<Arc<SigningKey>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
    let key = SigningKey::from_bytes(&bytes.try_into().map_err(|_| DeError::custom("Invalid key length"))?);
    Ok(Arc::new(key))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    #[serde(serialize_with = "serialize_signing_key", deserialize_with = "deserialize_signing_key")]
    pub private_address: Arc<SigningKey>,
    pub address: Address,
    pub name: String,
    pub visibility: bool,
    pub id: u64,
    pub state: NeuronStatus,
    pub staked: bool,
    pub staked_amount: u64,
    pub unlock_date: chrono::prelude::NaiveDate,
    pub age: chrono::prelude::NaiveDate,
    pub voting_power: u32,
    pub maturity: u64,
    pub bonus_multiplier: f64,
    pub date_created: chrono::DateTime<chrono::prelude::Utc>,
    pub dissolve_delay_bonus: u32,
    pub age_bonus: u32,
    pub total_bonus: u32,
    pub is_genesis: bool,
    pub is_known_neuron: bool,
    pub validator: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Transaction {
    #[serde(rename = "hash")]
    pub hash: String,

    #[serde(rename = "type")]
    pub r#type: TransactionType,

    #[serde(rename = "status")]
    pub status: TransactionStatus,

    #[serde(rename = "index")]
    pub index: u32,

    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime<chrono::prelude::Utc>,

    #[serde(rename = "from")]
    pub from: Address,

    #[serde(rename = "to")]
    pub to: Address,

    #[serde(rename = "amount")]
    pub amount: u64,

    #[serde(rename = "fee")]
    pub fee: u64,

    #[serde(rename = "memo")]
    pub memo: u32,

    #[serde(rename = "nrc_memo")]
    pub nrc_memo: u32,

    #[serde(rename = "signature")]
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub parent_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub validator: Address,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug)]
pub struct MutexWrapper<T: ?Sized>(pub Mutex<T>);

impl<T> MutexWrapper<T> {
    pub fn new(inner: T) -> Self {
        MutexWrapper(Mutex::new(inner))
    }
}

impl<T: ?Sized + Serialize> Serialize for MutexWrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let lock = self.0.lock();
        lock.serialize(serializer)
    }
}

impl<'de, T: ?Sized + Deserialize<'de>> Deserialize<'de> for MutexWrapper<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = T::deserialize(deserializer)?;
        Ok(MutexWrapper(Mutex::new(data)))
    }
}

impl<T: ?Sized> Deref for MutexWrapper<T> {
    type Target = Mutex<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for MutexWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone)]
pub struct DbWrapper(pub Arc<DB>);

impl Default for DbWrapper {
    fn default() -> Self {
        DbWrapper(Arc::new(DB::open_default("path_to_db").expect("Failed to open RocksDB")))
    }
}

impl Deref for DbWrapper {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.0).expect("Failed to get mutable reference")
    }
}