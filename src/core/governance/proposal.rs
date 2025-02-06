use std::collections::HashMap;
use chrono::{DateTime, Duration, Utc};
use std::cmp::Ordering;
use crate::core::types::{VotingStatus, VotingNeuron};

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

