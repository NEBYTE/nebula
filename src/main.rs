use crate::core::api::v1::transaction::{submit_transaction, build_transaction, finalize_transaction};
use crate::core::api::v1::wallet::create_wallet;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use crate::core::canister::registry::CanisterRegistry;
use crate::core::consensus::{ValidatorInfo};
use crate::core::consensus::model::ConsensusEngine;
use crate::core::nervous::NervousSystem;
use crate::core::types::TransactionType;

pub mod core {
   pub mod canister;
   pub mod consensus;
   pub mod api;
   pub mod governance;
   pub mod nervous;
   pub mod staking;
   pub mod crypto;
   pub mod types;
}

#[tokio::main]
async fn main() {
   let (signing_key, public_key, sender_address) = create_wallet();
   println!("Sender Wallet created: {:x?}", sender_address);

   let (_receiver_signing, receiver_public, receiver_address) = create_wallet();
   println!("Receiver Wallet created: {:x?}", receiver_address);

   let nervous_system = NervousSystem::new();
   let validators = vec![ValidatorInfo { address: sender_address.clone(), active: true }];
   let mut consensus_engine = ConsensusEngine::new(validators, nervous_system.neurons.clone());

   consensus_engine
       .init_ledger(sender_address.clone(), public_key, 100)
       .expect("Failed to initialize sender ledger");
   consensus_engine
       .init_ledger(receiver_address.clone(), receiver_public, 0)
       .expect("Failed to initialize receiver ledger");

   let mut canister_registry = CanisterRegistry::new();
   let canister_id = "my_canister".to_string();
   let canister = Canister::new(canister_id.clone(), sender_address.clone());

   println!("Canister created with ID: {}", canister.canister_id);
   canister_registry.register_canister(&canister_id, canister);

   let mut registered_canister = match canister_registry.get_canister(&canister_id) {
      Some(c) => c,
      None => {
         eprintln!("Canister '{}' not found", canister_id);
         return;
      }
   };

   {
      let amount = 50;

      let mut tx = build_transaction(
         &mut consensus_engine,
         sender_address.clone(),
         receiver_address.clone(),
         amount,
         0,
         0,
         TransactionType::Transfer,
      );
      finalize_transaction(&mut tx, &signing_key).expect("Failed to sign the transaction");

      let transfer_payload = CanisterFunctionPayload::Transfer {
         consensus_engine: &mut consensus_engine,
         tx,
      };

      match registered_canister.execute_function(transfer_payload) {
         Ok(msg) => println!("Successfully sent transfer: {}", msg),
         Err(err) => eprintln!("Transfer failed: {}", err),
      }
   }

   {
      let block = crate::core::consensus::block::produce_block(&mut consensus_engine, &signing_key)
          .expect("Failed to produce block");

      println!("Block produced with {} transaction(s)", block.transactions.len());
      println!("Block timestamp: {}", block.header.timestamp);
   }

   {
      let neuron_id = crate::core::nervous::neuron_handler::create_neuron(
         &nervous_system,
         &signing_key,
         "Test Neuron".to_string(),
         30,
      )
          .expect("Failed to create neuron");
      println!("Neuron created with id: {}", neuron_id);

      let mut staking_module =
          crate::core::staking::staking_module::StakingModule::new(nervous_system.neurons.clone());
      crate::core::staking::staking_handler::stake(&mut staking_module, &mut consensus_engine, &signing_key, neuron_id, 50)
          .expect("Failed to stake tokens");
      println!("Staked 50 tokens to neuron {}", neuron_id);

      let governance = crate::core::governance::Governance::new(nervous_system.neurons.clone());
      let proposal_id = crate::core::governance::proposal_handler::propose(
         &governance,
         "Increase block size".to_string(),
         &signing_key,
         neuron_id,
      )
          .expect("Failed to create proposal");
      println!("Proposal created with id: {}", proposal_id);
      match crate::core::governance::voting::vote(
         &governance,
         &signing_key,
         neuron_id,
         proposal_id,
         true,
         10,
      ) {
         Ok(_) => println!("Voted on proposal {}", proposal_id),
         Err(e) => println!("Voting failed: {}", e),
      }
      let proposal_result =
          crate::core::governance::voting::finalize(&governance, proposal_id).expect("Failed to finalize proposal");
      println!("Proposal finalized with result: {}", proposal_result);
   }
}
