use crate::core::consensus::block::produce_block;
use crate::core::consensus::model::ConsensusEngine;
use crate::core::consensus::validator::select_next_validator;
use crate::core::staking::StakingModule;
use crate::core::staking::distribute_rewards;
use crate::core::nervous::NervousSystem;

use ed25519_dalek::{SigningKey, VerifyingKey};
use tokio::time::{sleep, Duration, Instant};
use hex;

pub async fn run_consensus_loop(
    nervous_system: &mut NervousSystem,
    consensus_engine: &mut ConsensusEngine,
    staking_module: &mut StakingModule,
    signing_key: &SigningKey,
) {
    let target_cycle = Duration::from_millis(500);

    const REWARD_POOL: u64 = 50;
    const ANNUAL_YIELD_PERCENT: f64 = 5.0;

    loop {
        let cycle_start = Instant::now();

        if let Some(next_validator) = select_next_validator(consensus_engine) {
            let verifying_key = VerifyingKey::from(signing_key);
            let my_address = hex::encode(verifying_key.to_bytes());

            if next_validator == my_address {
                match produce_block(consensus_engine, signing_key) {
                    Ok(block) => {
                        println!(
                            "⚡ Block produced: {} transactions, timestamp: {}",
                            block.transactions.len(),
                            block.header.timestamp
                        );

                        distribute_rewards(nervous_system, &mut staking_module.clone(), REWARD_POOL, ANNUAL_YIELD_PERCENT);
                    }
                    Err(err) => {
                        eprintln!("Block production error: {}", err);
                    }
                }
            }
        }

        let elapsed = cycle_start.elapsed();
        let sleep_duration = if elapsed < target_cycle {
            target_cycle - elapsed
        } else {
            Duration::from_secs(1)
        };

        sleep(sleep_duration).await;
    }
}
