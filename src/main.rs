use std::error::Error;
use std::sync::Arc;
mod consensus;
mod staking;
mod governance;
mod api;
mod types;
mod crypto;
mod nervous;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let (signing_key, public_key, address) = api::create_wallet();
   // let (mock_signing_key, mock_public_key, mock_address) = api::create_wallet();
   // Uncomment above to test Neuron ownership

   println!("Wallet created: {:x?}", address);

   let mut nervous_system = nervous::NervousSystem::new();

   let validators = vec![consensus::ValidatorInfo { address: address.clone(), active: true }];
   let mut consensus_engine = consensus::ConsensusEngine::new(validators, nervous_system.neurons.clone());

   // let recipient = mock_address;
   // Uncomment above to test Neuron ownership

   let recipient = address.clone();
   let amount = 100;
   let mut tx = api::build_transaction(
      &mut consensus_engine,
      address.clone(),
      recipient.clone(),
      amount,
      0,
      0,
      types::TransactionType::Transfer,
   );
   api::finalize_transaction(&mut tx)?;
   api::submit_transaction(&mut consensus_engine, tx)?;

   let block = consensus_engine.produce_block(&signing_key)?;
   println!("Block produced with {} transaction(s)", block.transactions.len());
   println!("Block timestamp: {}", block.header.timestamp);

   let neuron_id = nervous_system.create_neuron(&signing_key, "Test Neuron".to_string(), 30)?;
   println!("Neuron created with id: {}", neuron_id);

   let mut staking_module = staking::StakingModule::new(nervous_system.neurons.clone());
   staking_module.stake(&signing_key, neuron_id, 50)?;
   println!("Staked 50 tokens to neuron {}", neuron_id);
   staking_module.unstake(&signing_key, neuron_id, 20)?;
   println!("Unstaked 20 tokens from neuron {}", neuron_id);

   let governance = governance::Governance::new(nervous_system.neurons.clone());
   {
      let mut neurons = governance.neurons.lock().unwrap();
      let neuron = types::Neuron {
         private_address: Arc::new(signing_key.clone()),
         name: "Test Neuron".to_string(),
         visibility: true,
         id: neuron_id,
         state: types::NeuronStatus::NotDissolving,
         staked: false,
         staked_amount: 100,
         dissolve_days: chrono::Utc::now().date_naive() + chrono::Duration::days(30),
         age: chrono::Utc::now().date_naive(),
         voting_power: 100,
         date_created: chrono::Utc::now(),
         dissolve_delay_bonus: 0,
         age_bonus: 0,
         total_bonus: 0,
         is_genesis: false,
         is_known_neuron: false,
         validator: None,
      };
      neurons.insert(neuron_id, neuron);
   }

   // staking_module.stake(&mock_signing_key, neuron_id, 50)?;
   // Uncomment above to test Neuron ownership

   let proposal_id = governance.propose("Increase block size".to_string(), &signing_key, neuron_id)?;
   println!("Proposal created with id: {}", proposal_id);

   governance.vote(&signing_key, neuron_id, proposal_id, true, 10)?;
   println!("Voted on proposal {}", proposal_id);

   let proposal_result = governance.finalize(proposal_id)?;
   println!("Proposal finalized with result: {}", proposal_result);

   Ok(())
}
