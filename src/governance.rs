/// Governance for Nebula.
/// Enables proposals, voting, and finalizing.
use crate::types::Address;

pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub proposer: Address,
}

pub struct Governance {
    pub proposals: Vec<Proposal>,
    pub next_id: u64,
}

impl Governance {
    pub fn new() -> Self {
        Self { proposals: vec![], next_id: 1 }
    }

    pub fn propose(&mut self, description: String, proposer: Address) -> u64 {
        let p = Proposal {
            id: self.next_id,
            description,
            votes_for: 0,
            votes_against: 0,
            proposer,
        };
        self.proposals.push(p);
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn vote(&mut self, proposal_id: u64, vote_for: bool, stake: u64) -> Result<(), String> {
        let prop = self.proposals.iter_mut().find(|x| x.id == proposal_id).ok_or("Not found")?;
        if vote_for {
            prop.votes_for += stake;
        } else {
            prop.votes_against += stake;
        }
        Ok(())
    }

    pub fn finalize(&self, proposal_id: u64) -> Option<bool> {
        let prop = self.proposals.iter().find(|x| x.id == proposal_id)?;
        Some(prop.votes_for > prop.votes_against)
    }
}
