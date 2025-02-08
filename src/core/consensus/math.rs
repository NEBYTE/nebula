pub fn consensus_probability(s_i: u64, s_total: u64) -> f64 {
    if s_total == 0 {
        0.0
    } else {
        s_i as f64 / s_total as f64
    }
}

pub fn staking_yield(stake: u64, yield_percent: f64) -> u64 {
    ((stake as f64) * (yield_percent / 100.0)).round() as u64
}

pub fn block_time(lambda: f64) -> f64 {
    if lambda == 0.0 { 0.0 } else { 1.0 / lambda }
}

pub fn voting_power(stake: u64, bonus_multiplier: f64) -> u64 {
    ((stake as f64) * bonus_multiplier).round() as u64
}

pub fn governance_voting_outcome(votes: &[(u64, i64)]) -> i64 {
    votes.iter().map(|(v_power, vote_val)| (*v_power as i64) * vote_val).sum()
}
