use crate::core::consensus::math::staking_yield;
use crate::core::nervous::NervousSystem;
use crate::core::staking::StakingModule;

pub fn distribute_rewards(
    nervous_system: &mut NervousSystem,
    staking_module: &mut StakingModule,
    reward_pool: u64,
    annual_yield_percent: f64,
) {
    {
        let mut neurons = staking_module.neurons.lock();
        let total_staked: u64 = neurons.values().map(|n| n.staked_amount).sum();

        if total_staked == 0 {
            return;
        }

        let total_staked_f64 = total_staked as f64;

        for neuron in neurons.values_mut() {
            let ratio = neuron.staked_amount as f64 / total_staked_f64;
            let maturity_bonus = 1.0 + (neuron.maturity as f64 / 100.0);
            let pool_reward = (reward_pool as f64 * ratio) * maturity_bonus;

            let yield_reward = staking_yield(neuron.staked_amount, annual_yield_percent);
            let total_reward = pool_reward.round() as u64 + yield_reward;

            neuron.staked_amount += total_reward;
            neuron.maturity += 1;
        }
    }

    nervous_system.persist_neurons();
}