use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, RwLock, Mutex};
use chrono::{DateTime, Duration, Utc};
use crate::types::{Address, VotingStatus, VotingNeuron, Neuron, Vote};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tally {
    pub yes: u64,
    pub no: u64,
    pub total: u64,
}

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub topic: String,
    pub status: VotingStatus,
    pub r#type: String,
    pub reward_status: String,
    pub reward_height: u64,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub proposer_id: u64,
    pub url: String,
    pub reject_cost: u64,
    pub rejudge_cost: u64,
    pub votes_of_known_neurons: HashMap<u64, VotingNeuron>,
    pub votes_of_neurons: HashMap<u64, VotingNeuron>,
    pub payload: Vec<u8>,
    pub summary: String,
    pub voting_period_remaining: Duration,
    pub voting_period_start: DateTime<Utc>,
    pub voting_period_end: DateTime<Utc>,
    pub tally: Tally,
}

use std::cmp::Ordering;
impl PartialOrd for Proposal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Proposal {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_net = self.tally.yes as i64 - self.tally.no as i64;
        let other_net = other.tally.yes as i64 - other.tally.no as i64;
        self_net.cmp(&other_net)
    }
}

impl PartialEq for Proposal {
    fn eq(&self, other: &Self) -> bool {
        (self.tally.yes as i64 - self.tally.no as i64) == (other.tally.yes as i64 - other.tally.no as i64)
    }
}

impl Eq for Proposal {}

pub struct Governance {
    pub proposals: Arc<RwLock<BinaryHeap<Proposal>>>,
    pub neurons: Arc<Mutex<HashMap<u64, Neuron>>>,
    pub total_voting_power: u128,
    pub daily_voting_rewards: u128,
    pub next_id: Arc<Mutex<u64>>,
}

impl Governance {
    pub fn new() -> Self {
        Self {
            proposals: Arc::new(RwLock::new(BinaryHeap::new())),
            neurons: Arc::new(Mutex::new(HashMap::new())),
            total_voting_power: 500_000_000,
            daily_voting_rewards: 90_500,
            next_id: Arc::new(Mutex::new(1)), // genesis is 0 soo real proposal is 1
        }
    }

    pub fn propose(&self, topic: String, caller: Address, proposer_id: u64) -> Result<u64, String> {
        let now = Utc::now();

        let neurons = self.neurons.lock().unwrap();
        let neuron = neurons.get(&proposer_id).ok_or("Proposer does not own a neuron")?;

        if neuron.private_address != caller {
            return Err("Caller does not own the neuron".to_string());
        }

        let mut next_id = self.next_id.lock().unwrap();
        let proposal_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let proposal = Proposal {
            id: proposal_id,
            topic,
            status: VotingStatus::Open,
            r#type: String::from("default"),
            reward_status: String::from("none"),
            reward_height: 0,
            date_created: now,
            date_modified: now,
            proposer_id,
            url: String::new(),
            reject_cost: 0,
            rejudge_cost: 0,
            votes_of_known_neurons: HashMap::new(),
            votes_of_neurons: HashMap::new(),
            payload: Vec::new(),
            summary: String::new(),
            voting_period_remaining: Duration::seconds(3600),
            voting_period_start: now,
            voting_period_end: now + Duration::seconds(3600),
            tally: Tally { yes: 0, no: 0, total: 0 },
        };

        let mut heap = self.proposals.write().unwrap();
        heap.push(proposal);
        Ok(proposal_id)
    }

    pub fn vote(&self, caller: Address, neuron_id: u64, proposal_id: u64, vote_for: bool, stake: u64) -> Result<(), String> {
        let mut neurons = self.neurons.lock().unwrap();
        let neuron = neurons.get_mut(&neuron_id).ok_or("Proposer does not own a neuron")?;

        if neuron.private_address != caller {
            return Err("Caller does not own the neuron".to_string());
        }

        let mut heap = self.proposals.write().unwrap();
        let mut temp = Vec::new();
        let mut found = None;

        while let Some(mut proposal) = heap.pop() {
            if proposal.id == proposal_id {

                if proposal.votes_of_neurons.contains_key(&neuron_id) {
                    return Err("Neuron has already voted on this proposal".to_string());
                }


                let vote = if vote_for { Vote::Yes } else { Vote::No };
                let voting_neuron = VotingNeuron {
                    name: neuron.name.clone(),
                    id: neuron.id,
                    vote,
                    private_address: caller,
                };

                proposal.votes_of_neurons.insert(neuron_id, voting_neuron);

                if vote_for {
                    proposal.tally.yes += stake;
                } else {
                    proposal.tally.no += stake;
                }
                proposal.tally.total += stake;
                found = Some(proposal);
                break;
            } else {
                temp.push(proposal);
            }
        }

        for p in temp {
            heap.push(p);
        }

        if let Some(proposal) = found {
            heap.push(proposal);
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }

    pub fn finalize(&self, proposal_id: u64) -> Result<bool, String> {
        let mut heap = self.proposals.write().unwrap();
        let mut temp = Vec::new();
        let mut finalized = None;

        while let Some(mut proposal) = heap.pop() {
            if proposal.id == proposal_id {
                proposal.status = VotingStatus::Terminated;
                finalized = Some(proposal.tally.yes > proposal.tally.no);
                break;
            } else {
                temp.push(proposal);
            }
        }

        for p in temp {
            heap.push(p);
        }

        match finalized {
            Some(result) => Ok(result),
            None => Err("Proposal not found".to_string()),
        }
    }

    pub fn list_proposals(&self) -> Vec<Proposal> {
        let heap = self.proposals.read().unwrap();
        heap.clone().into_sorted_vec()
    }
}
