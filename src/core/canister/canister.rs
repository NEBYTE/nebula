use crate::core::types::{Address, MutexWrapper, Transaction, DbWrapper};
use crate::core::consensus::*;
use crate::core::staking::*;
use crate::core::governance::{
    proposal_handler::propose,
    voting::{vote, finalize},
    Governance,
};
use crate::core::nervous::*;
use crate::core::consensus::model::ConsensusEngine;
use crate::core::consensus::transaction::cancel_transaction;

use std::collections::HashMap;
use std::sync::Arc;
use ed25519_dalek::SigningKey;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub enum CanisterFunctionPayload<'a> {
    Transfer {
        consensus_engine: &'a mut ConsensusEngine,
        tx: Transaction,
    },
    Stake {
        nervous_system: &'a mut NervousSystem,
        staking_module: &'a mut StakingModule,
        consensus_engine: &'a mut ConsensusEngine,
        signing_key: &'a SigningKey,
        neuron_id: u64,
        amount: u64,
    },
    Unstake {
        nervous_system: &'a mut NervousSystem,
        staking_module: &'a mut StakingModule,
        consensus_engine: &'a mut ConsensusEngine,
        signing_key: &'a SigningKey,
        neuron_id: u64,
        amount: u64,
    },
    Propose {
        governance: &'a mut Governance,
        topic: String,
        signing_key: &'a SigningKey,
        neuron_id: u64,
    },
    Vote {
        governance: &'a mut Governance,
        signing_key: &'a SigningKey,
        neuron_id: u64,
        proposal_id: u64,
        vote: bool,
        stake_amount: u64,
    },
    ProduceBlock {
        consensus_engine: &'a mut ConsensusEngine,
        signing_key: &'a SigningKey,
    },
    SelectValidator {
        consensus_engine: &'a mut ConsensusEngine,
    },
    CreateNeuron {
        nervous_system: &'a mut NervousSystem,
        signing_key: &'a SigningKey,
        name: String,
        dissolve_days: i64,
    },
    GetNeuron {
        nervous_system: &'a mut NervousSystem,
        neuron_id: u64,
    },
    ListNeurons {
        nervous_system: &'a mut NervousSystem,
    },
    Finalize {
        governance: &'a mut Governance,
        proposal_id: u64,
    },
    CancelTransfer {
        consensus_engine: &'a mut ConsensusEngine,
        tx_hash: String,
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Canister {
    pub state: Arc<MutexWrapper<HashMap<String, String>>>,
    pub canister_id: String,
    pub controller: Address,
    pub module_hash: String,
    #[serde(skip)]
    pub db: DbWrapper,
}

impl Canister {
    pub fn new(canister_id: String, controller: Address, db: Arc<DB>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(canister_id.as_bytes());
        hasher.update(controller.to_string().as_bytes());

        let hash_result = hasher.finalize();
        let module_hash = format!("{:x}", hash_result);

        Self {
            state: Arc::new(MutexWrapper::new(HashMap::new())),
            canister_id,
            controller,
            module_hash,
            db: DbWrapper(db),
        }
    }

    pub fn execute_function<'a>(
        &mut self,
        payload: CanisterFunctionPayload<'a>,
    ) -> Result<String, String> {
        match payload {
            CanisterFunctionPayload::Transfer { consensus_engine, tx } => {
                add_transaction(consensus_engine, tx.clone())?;
                Ok(format!(
                    "Transaction executed: {} -> {} ({} tokens)",
                    tx.from, tx.to, tx.amount
                ))
            }
            CanisterFunctionPayload::CancelTransfer { consensus_engine, tx_hash } => {
                cancel_transaction(consensus_engine, tx_hash.clone())?;
                Ok("Transacted cancelled!".to_string())
            }
            CanisterFunctionPayload::Stake {
                nervous_system,
                staking_module,
                consensus_engine,
                signing_key,
                neuron_id,
                amount,
            } => {
                stake(nervous_system, staking_module, consensus_engine, signing_key, neuron_id, amount)?;
                Ok(format!(
                    "Stake executed: {} -> {} tokens staked",
                    neuron_id, amount
                ))
            }
            CanisterFunctionPayload::Unstake {
                nervous_system,
                staking_module,
                consensus_engine,
                signing_key,
                neuron_id,
                amount,
            } => {
                unstake(nervous_system, staking_module, consensus_engine, signing_key, neuron_id, amount)?;
                Ok(format!(
                    "Unstake executed: Neuron {} -> {} tokens unstaked",
                    neuron_id, amount
                ))
            }
            CanisterFunctionPayload::Propose {
                governance,
                topic,
                signing_key,
                neuron_id,
            } => {
                let proposal_id = propose(governance, topic, signing_key, neuron_id)?;
                Ok(format!(
                    "Proposal '{}' created by neuron {}",
                    proposal_id, neuron_id
                ))
            }
            CanisterFunctionPayload::Vote {
                governance,
                signing_key,
                neuron_id,
                proposal_id,
                vote: vote_value,
                stake_amount,
            } => {
                vote(governance, signing_key, neuron_id, proposal_id, vote_value, stake_amount)?;
                Ok(format!(
                    "Neuron {} voted '{:?}' on proposal {}",
                    neuron_id, vote_value, proposal_id
                ))
            }
            CanisterFunctionPayload::ProduceBlock {
                consensus_engine,
                signing_key,
            } => {
                let block = produce_block(consensus_engine, signing_key)?;
                Ok(format!(
                    "Block produced: {} transactions, timestamp: {}",
                    block.transactions.len(),
                    block.header.timestamp
                ))
            }
            CanisterFunctionPayload::SelectValidator { consensus_engine } => {
                if let Some(validator) = select_next_validator(consensus_engine) {
                    Ok(format!("Selected validator: {}", validator))
                } else {
                    Err("No validator selected".to_string())
                }
            }
            CanisterFunctionPayload::CreateNeuron {
                nervous_system,
                signing_key,
                name,
                dissolve_days,
            } => {
                let neuron_id = create_neuron(nervous_system, signing_key, name, dissolve_days)?;
                Ok(format!("Neuron created with ID: {}", neuron_id))
            }
            CanisterFunctionPayload::GetNeuron {
                nervous_system,
                neuron_id,
            } => {
                if let Some(neuron) = get_neuron(nervous_system, neuron_id) {
                    Ok(format!("Neuron {}: {:?}", neuron_id, neuron))
                } else {
                    Err(format!("Neuron {} not found", neuron_id))
                }
            }
            CanisterFunctionPayload::ListNeurons { nervous_system } => {
                let neurons = list_neurons(nervous_system);
                Ok(format!("Neurons: {:?}", neurons))
            }
            CanisterFunctionPayload::Finalize {
                governance,
                proposal_id,
            } => {
                finalize(governance, proposal_id)?;
                Ok(format!("Finalized proposal: {}", proposal_id))
            }
        }
    }

    pub fn persist_state(&self) {
        let state = self.state.lock();
        let serialized = bincode::serialize(&*state).unwrap();
        let key = format!("canister_state_{}", self.canister_id);
        self.db.put(key.as_bytes(), serialized).unwrap();
    }

    pub fn store_state(&mut self, key: String, value: String) {
        let mut state = self.state.lock();
        state.insert(key, value);

        drop(state);
        self.persist_state();
    }

    pub fn load_state(&mut self) {
        let key = format!("canister_state_{}", self.canister_id);
        if let Ok(Some(data)) = self.db.get(key.as_bytes()) {
            let loaded_state: HashMap<String, String> = bincode::deserialize(&data).unwrap();
            *self.state.lock() = loaded_state;
        }
    }

    pub fn canister_info(&mut self) -> String {
        format!("Canister ID: {}, Controller: {}, Module Hash: {}", self.canister_id, self.controller, self.module_hash)
    }
}
