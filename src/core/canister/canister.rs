use crate::core::types::{Address, Transaction};
use crate::core::consensus::*;
use crate::core::staking::*;
use crate::core::governance::{
    proposal_handler::propose,
    voting::{vote, finalize},
    Governance,
};
use crate::core::nervous::*;
use crate::core::consensus::model::ConsensusEngine;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ed25519_dalek::SigningKey;
use sha2::{Digest, Sha256};

pub enum CanisterFunctionPayload<'a> {
    Transfer {
        consensus_engine: &'a mut ConsensusEngine,
        tx: Transaction,
    },
    Stake {
        staking_module: &'a mut StakingModule,
        consensus_engine: &'a mut ConsensusEngine,
        signing_key: &'a SigningKey,
        neuron_id: u64,
        amount: u64,
    },
    Unstake {
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
}

#[derive(Clone)]
pub struct Canister {
    pub state: Arc<Mutex<HashMap<String, String>>>,
    pub canister_id: String,
    pub controller: Address,
    pub module_hash: String,
}

impl Canister {
    pub fn new(canister_id: String, controller: Address) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(canister_id.as_bytes());
        hasher.update(controller.to_string().as_bytes());

        let hash_result = hasher.finalize();
        let module_hash = format!("{:x}", hash_result);

        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            canister_id,
            controller,
            module_hash,
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
            CanisterFunctionPayload::Stake {
                staking_module,
                consensus_engine
                signing_key,
                neuron_id,
                amount,
            } => {
                stake(staking_module, consensus_engine, signing_key, neuron_id, amount)?;
                Ok(format!(
                    "Stake executed: {} -> {} tokens staked",
                    neuron_id, amount
                ))
            }
            CanisterFunctionPayload::Unstake {
                staking_module,
                consensus_engine,
                signing_key,
                neuron_id,
                amount,
            } => {
                unstake(staking_module, consensus_engine, signing_key, neuron_id, amount)?;
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

    pub fn store_state(&mut self, key: String, value: String) {
        let mut state = self.state.lock().unwrap();
        state.insert(key, value);
    }

    pub fn load_state(&self, key: &str) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.get(key).cloned()
    }

    pub fn canister_info(&self) -> String {
        format!("Canister ID: {}, Controller: {}, Module Has: {}", self.canister_id, self.controller, self.module_hash)
    }
}
