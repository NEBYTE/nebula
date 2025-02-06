use crate::core::staking::StakingModule;

pub fn distribute_rewards(
    staking_module: &mut StakingModule,
    reward_pool: u64
) {
    let mut neurons = staking_module.neurons.lock().unwrap();
    let total_staked: u64 = neurons.values().map(|n| n.staked_amount).sum();

    if total_staked == 0 {
        return;
    }

    for neuron in neurons.values_mut() {
        let ratio = neuron.staked_amount as f64 / total_staked as f64;
        let maturity_bonus = 1.0 + (neuron.maturity as f64 / 100.0);
        let reward = ((reward_pool as f64 * ratio) * maturity_bonus) as u64;
        neuron.staked_amount += reward;
        neuron.maturity += 1;
    }
}