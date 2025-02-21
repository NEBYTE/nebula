use std::collections::HashMap;
use std::sync::{Arc};
use ed25519_dalek::SigningKey;
use chrono::{Duration, Utc};
use crate::core::types::{VotingStatus};
use crate::core::governance::proposal::{Proposal, Tally};
use crate::core::governance::governance::Governance;

pub fn propose(
    governance: &Governance,
    topic: String,
    caller: &SigningKey,
    proposer_id: u64
) -> Result<u64, String> {
    let now = Utc::now();

    let neurons = governance.neurons.lock();
    let neuron = neurons.get(&proposer_id).ok_or("Proposer does not own a neuron")?;

    if neuron.private_address != Arc::new(caller.clone()) {
        return Err("Caller does not own the neuron".to_string());
    }

    let mut next_id = governance.next_id.lock();
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

    let mut heap = governance.proposals.write().unwrap();
    heap.push(proposal);

    drop(heap);
    drop(neurons);

    governance.persist_proposals();
    Ok(proposal_id)
}

pub fn list_proposals(governance: &Governance) -> Vec<Proposal> {
    let heap = governance.proposals.read().map_err(|_| "RwLock poisoned unfortunatly").unwrap();
    heap.clone().into_sorted_vec()
}