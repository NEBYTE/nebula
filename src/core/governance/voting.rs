use crate::core::consensus::math::voting_power;
use crate::core::types::{Vote, VotingNeuron, VotingStatus};
use crate::core::governance::governance::Governance;

use std::sync::{Arc};
use ed25519_dalek::SigningKey;

pub fn vote(
    governance: &Governance,
    caller: &SigningKey,
    neuron_id: u64,
    proposal_id: u64,
    vote_for: bool,
    stake: u64
) -> Result<(), String> {
    {
        let mut neurons = governance.neurons.lock();
        let neuron = neurons.get_mut(&neuron_id).ok_or("Proposer does not own a neuron")?;

        if neuron.private_address != Arc::new(caller.clone()) {
            return Err("Caller does not own the neuron".to_string());
        }

        if neuron.staked_amount < stake {
            return Err("Neuron does not have enough stake to vote".to_string())
        }

        let mut heap = governance.proposals.write().unwrap();
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
                    private_address: Arc::new(caller.clone()),
                };

                proposal.votes_of_neurons.insert(neuron_id, voting_neuron);

                let effective_stake = voting_power(stake, neuron.bonus_multiplier);
                if vote_for {
                    proposal.tally.yes += effective_stake;
                } else {
                    proposal.tally.no += effective_stake;
                }
                proposal.tally.total += effective_stake;
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
        } else {
            return Err("Proposal not found".to_string()).expect("Couldn't return an error message");
        }
    }

    governance.persist_proposals();
    Ok(())
}

pub fn finalize(
    governance: &Governance,
    proposal_id: u64
) -> Result<bool, String> {
    let mut heap = governance.proposals.write().unwrap();
    let mut temp = Vec::new();
    let mut finalized = None;

    while let Some(mut proposal) = heap.pop() {
        if proposal.id == proposal_id {
            proposal.status = VotingStatus::Terminated;

            let outcome = governance.compute_voting_outcome(&proposal);
            finalized = Some(outcome > 0);

            break;
        } else {
            temp.push(proposal);
        }
    }

    for p in temp {
        heap.push(p);
    }

    match finalized {
        Some(result) => {
            drop(heap);
            governance.persist_proposals();
            Ok(result)
        },
        None => Err("Proposal not found".to_string()),
    }
}
