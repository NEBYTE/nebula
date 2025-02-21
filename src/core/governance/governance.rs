use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, RwLock};
use rocksdb::DB;
use crate::core::consensus::math::voting_power;
use crate::core::types::{Neuron, Vote, MutexWrapper};
use crate::core::governance::proposal::Proposal;

pub struct Governance {
    pub proposals: Arc<RwLock<BinaryHeap<Proposal>>>,
    pub neurons: Arc<MutexWrapper<HashMap<u64, Neuron>>>,
    pub daily_voting_rewards: Arc<MutexWrapper<u128>>,
    pub next_id: Arc<MutexWrapper<u64>>,
    pub db: Arc<DB>,
}

impl Governance {
    pub fn new(neurons: Arc<MutexWrapper<HashMap<u64, Neuron>>>, db: Arc<DB>) -> Self {
        let governance = Self {
            proposals: Arc::new(RwLock::new(BinaryHeap::new())),
            neurons,
            daily_voting_rewards: Arc::new(MutexWrapper::new(90_500)),
            next_id: Arc::new(MutexWrapper::new(1)),
            db,
        };
        governance.load_state();
        governance
    }

    pub fn load_state(&self) {
        {
            let mut proposals_lock = self.proposals.write().unwrap();
            proposals_lock.clear();
            let iter = self.db.iterator(rocksdb::IteratorMode::Start);
            for item in iter {
                let (key, value) = item.unwrap();
                if key.starts_with(b"proposal_") {
                    if let Ok(proposal) = bincode::deserialize::<Proposal>(&value) {
                        proposals_lock.push(proposal);
                    } else {
                        eprintln!("Failed to deserialize proposal with key {:?}.", key);
                    }
                }
            }
        }
        if let Ok(Some(val)) = self.db.get(b"governance_daily_voting_rewards") {
            if let Ok(dvr) = bincode::deserialize::<u128>(&val) {
                *self.daily_voting_rewards.lock() = dvr;
            } else {
                eprintln!("Failed to load daily_voting_rewards.");
            }
        }
        if let Ok(Some(val)) = self.db.get(b"governance_next_id") {
            if let Ok(nid) = bincode::deserialize::<u64>(&val) {
                *self.next_id.lock() = nid;
            } else {
                eprintln!("Failed to load next_id.");
            }
        }
    }

    pub fn persist_state(&self) {
        self.persist_proposals();

        let dvr_serialized = bincode::serialize(&*self.daily_voting_rewards.lock()).unwrap();
        self.db.put(b"governance_daily_voting_rewards", dvr_serialized).unwrap();
        let nid_serialized = bincode::serialize(&*self.next_id.lock()).unwrap();
        self.db.put(b"governance_next_id", nid_serialized).unwrap();
    }

    pub fn persist_proposals(&self) {
        let proposals = self.proposals.read().unwrap();
        for proposal in proposals.iter() {
            let serialized = bincode::serialize(proposal).unwrap();
            let key = format!("proposal_{}", proposal.id);
            self.db.put(key.as_bytes(), serialized).unwrap();
        }
    }

    pub fn compute_voting_outcome(&self, proposal: &Proposal) -> i64 {
        let neurons = self.neurons.lock();
        let mut outcome: i64 = 0;
        for (&neuron_id, voting_neuron) in proposal.votes_of_neurons.iter() {
            let vote_value = match voting_neuron.vote {
                Vote::Yes => 1,
                Vote::No => -1,
                _ => 0,
            };

            if let Some(neuron) = neurons.get(&neuron_id) {
                let effective_power = voting_power(neuron.staked_amount, neuron.bonus_multiplier);
                outcome += (effective_power as i64) * vote_value;
            }
        }
        outcome
    }
}
