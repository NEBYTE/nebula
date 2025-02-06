use std::error::Error;

pub mod core {
   pub mod api {
      pub mod v1;
   }

   pub mod governance;
   pub mod staking;
   pub mod consensus;
   pub mod nervous;
   pub mod types;
   pub mod crypto;
}

use crate::core::governance::*;
use crate::core::api::v1::*;
use crate::core::consensus::*;
use crate::core::nervous::*;
use crate::core::staking::staking_module::StakingModule;
use crate::core::staking::staking_handler::{stake, unstake};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let (signing_key, _public_key, address) = create_wallet();
   println!("Wallet created: {:x?}", address);

   let nervous_system = NervousSystem::new();
   let validators = vec![ValidatorInfo { address: address.clone(), active: true }];
   let mut consensus_engine = ConsensusEngine::new(validators, nervous_system.neurons.clone());

   let recipient = address.clone();
   let amount = 100;

   let mut tx = build_transaction(
      &mut consensus_engine,
      address.clone(),
      recipient.clone(),
      amount,
      0,
      0,
      core::types::TransactionType::Transfer,
   );

   finalize_transaction(&mut tx, &signing_key)?;
   submit_transaction(&mut consensus_engine, tx)?;

   let block = produce_block(&mut consensus_engine, &signing_key)?;
   println!("Block produced with {} transaction(s)", block.transactions.len());
   println!("Block timestamp: {}", block.header.timestamp);

   let neuron_id = create_neuron(&nervous_system, &signing_key, "Test Neuron".to_string(), 30)?;
   println!("Neuron created with id: {}", neuron_id);

   let mut staking_module = StakingModule::new(nervous_system.neurons.clone());
   stake(&mut staking_module, &signing_key, neuron_id, 50)?;
   println!("Staked 50 tokens to neuron {}", neuron_id);

   match unstake(&mut staking_module, &signing_key, neuron_id, 20) {
      Ok(_) => println!("Unstaked 20 tokens from neuron {}", neuron_id),
      Err(e) => println!("Unstaking failed: {}", e),
   }

   let governance = Governance::new(nervous_system.neurons.clone());

   let neuron_id = create_neuron(&nervous_system, &signing_key, "Test Neuron1".to_string(), 30)?;
   stake(&mut staking_module, &signing_key, neuron_id, 50)?;

   let proposal_id = propose(&governance, "Increase block size".to_string(), &signing_key, neuron_id)?;
   println!("Proposal created with id: {}", proposal_id);

   match vote(&governance, &signing_key, neuron_id, proposal_id, true, 10) {
      Ok(_) => println!("Voted on proposal {}", proposal_id),
      Err(e) => println!("Voting failed: {}", e),
   }

   let proposal_result = finalize(&governance, proposal_id)?;
   println!("Proposal finalized with result: {}", proposal_result);

   Ok(())
}
