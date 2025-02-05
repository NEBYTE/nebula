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


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let (signing_key, public_key, address) = core::api::v1::create_wallet();
   println!("Wallet created: {:x?}", address);

   let nervous_system = core::nervous::NervousSystem::new();
   let validators = vec![core::consensus::ValidatorInfo { address: address.clone(), active: true }];
   let mut consensus_engine = core::consensus::ConsensusEngine::new(validators, nervous_system.neurons.clone());

   let recipient = address.clone();
   let amount = 100;

   let mut tx = core::api::v1::build_transaction(
      &mut consensus_engine,
      address.clone(),
      recipient.clone(),
      amount,
      0,
      0,
      core::types::TransactionType::Transfer,
   );

   core::api::v1::finalize_transaction(&mut tx, &signing_key)?;
   core::api::v1::submit_transaction(&mut consensus_engine, tx)?;

   let block = consensus_engine.produce_block(&signing_key)?;
   println!("Block produced with {} transaction(s)", block.transactions.len());
   println!("Block timestamp: {}", block.header.timestamp);

   let neuron_id = nervous_system.create_neuron(&signing_key, "Test Neuron".to_string(), 30)?;
   println!("Neuron created with id: {}", neuron_id);

   let mut staking_module = core::staking::StakingModule::new(nervous_system.neurons.clone());
   staking_module.stake(&signing_key, neuron_id, 50)?;
   println!("Staked 50 tokens to neuron {}", neuron_id);

   match staking_module.unstake(&signing_key, neuron_id, 20) {
      Ok(_) => println!("Unstaked 20 tokens from neuron {}", neuron_id),
      Err(e) => println!("Unstaking failed: {}", e),
   }

   let governance = core::governance::Governance::new(nervous_system.neurons.clone());

   let neuron_id = nervous_system.create_neuron(&signing_key, "Test Neuron1".to_string(), 30)?;
   staking_module.stake(&signing_key, neuron_id, 50)?;

   let proposal_id = governance.propose("Increase block size".to_string(), &signing_key, neuron_id)?;
   println!("Proposal created with id: {}", proposal_id);

   match governance.vote(&signing_key, neuron_id, proposal_id, true, 10) {
      Ok(_) => println!("Voted on proposal {}", proposal_id),
      Err(e) => println!("Voting failed: {}", e),
   }

   let proposal_result = governance.finalize(proposal_id)?;
   println!("Proposal finalized with result: {}", proposal_result);

   Ok(())
}
