/// Staking for Nebula.
/// Lets accounts stake, unstake, and get rewards.
use crate::types::Address;

#[derive(Clone)]
pub struct StakingAccount {
    pub address: Address,
    pub balance: u64,
    pub staked: u64,
}

pub struct StakingModule {
    pub accounts: Vec<StakingAccount>,
}

impl StakingModule {
    pub fn new() -> Self {
        Self { accounts: vec![] }
    }

    pub fn stake(&mut self, address: Address, amount: u64) -> Result<(), String> {
        let acc = self.accounts.iter_mut().find(|a| a.address == address).ok_or("No account")?;
        if acc.balance < amount {
            return Err("Insufficient balance".into());
        }
        acc.balance -= amount;
        acc.staked += amount;
        Ok(())
    }

    pub fn unstake(&mut self, address: Address, amount: u64) -> Result<(), String> {
        let acc = self.accounts.iter_mut().find(|a| a.address == address).ok_or("No account")?;
        if acc.staked < amount {
            return Err("Insufficient stake".into());
        }
        acc.staked -= amount;
        acc.balance += amount;
        Ok(())
    }

    pub fn distribute_rewards(&mut self, reward_pool: u64) {
        let total_staked: u64 = self.accounts.iter().map(|a| a.staked).sum();
        if total_staked == 0 { return; }
        for acc in &mut self.accounts {
            let ratio = acc.staked as f64 / total_staked as f64;
            let reward = (reward_pool as f64 * ratio) as u64;
            acc.balance += reward;
        }
    }
}
