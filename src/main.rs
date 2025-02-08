use std::time::Duration;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use tokio::task;
use crate::core::api::v1::transaction::{build_transaction, finalize_transaction};
use crate::core::api::v1::wallet::create_wallet;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use crate::core::canister::registry::CanisterRegistry;
use crate::core::consensus::{delegate_stake, ValidatorInfo};
use crate::core::consensus::consensus::run_consensus_loop;
use crate::core::consensus::model::ConsensusEngine;
use crate::core::consensus::validator::{build_validator, register_validator, wrap_validator};
use crate::core::governance::Governance;
use crate::core::nervous::{create_neuron, NervousSystem};
use crate::core::staking::{stake, StakingModule};
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

   let mut nervous_system = NervousSystem::new();
   let mut staking_module = StakingModule::new(nervous_system.neurons.clone());

   let neuron_id = create_neuron(&mut nervous_system, &signing_key, "John Doe".to_string(), 365).unwrap();
   println!("Created Neuron with ID: {}", neuron_id);

   let mut built_validator = build_validator(&mut nervous_system, neuron_id.clone()).unwrap();
   built_validator.active = true;

   let wrapped_validators = wrap_validator(built_validator);

   let mut consensus_engine = ConsensusEngine::new(wrapped_validators, nervous_system.neurons.clone());
   let mut governance_module = Governance::new(nervous_system.neurons.clone());
   let mut canister_registry = CanisterRegistry::new();

   let sender_ledger_wallet = consensus_engine.init_ledger(sender_address.clone(), public_key, 1000);
   let receiver_ledger_wallet = consensus_engine.init_ledger(receiver_address.clone(), public_key, 1000);

   stake(&mut staking_module, &mut consensus_engine, &signing_key, neuron_id, 500).expect("Failed to stake 500 tokens");
   delegate_stake(&mut consensus_engine, neuron_id, sender_address.clone()).expect("Failed to delegate stake to neuron");

   let target_cycle = Duration::from_secs(1/2);

   let signing_key_clone = signing_key.clone();
   let mut consensus_engine_clone = consensus_engine.clone();

   tokio::spawn(async move {
      run_consensus_loop(
         &mut consensus_engine_clone,
         &signing_key_clone,
         target_cycle,
      ).await;
   });

   println!("🚀 Blockchain node is running! Listening for transactions...");

   let canister = Canister::new("my_canister".to_string(), sender_address.clone());
   println!("Canister created with ID: {}", canister.canister_id);

   canister_registry.register_canister(&canister.canister_id, canister.clone());

   let mut registered_canister = match canister_registry.get_canister(&canister.canister_id) {
      Some(c) => c,
      None => {
         eprintln!("Canister '{}' not found", &canister.canister_id);
         return;
      }
   };

   let mut tx = build_transaction(
      &mut consensus_engine,
      sender_address.clone(),
      receiver_address.clone(),
      50,
      0,
      0,
      TransactionType::Transfer,
   );

   finalize_transaction(&mut tx, &signing_key).expect("Failed to finalize transaction");

   let transfer_payload = CanisterFunctionPayload::Transfer {
      consensus_engine: &mut consensus_engine,
      tx,
   };

   match registered_canister.execute_function(transfer_payload) {
      Ok(msg) => println!("Successfully sent transfer: {}", msg),
      Err(err) => eprintln!("Transfer failed: {}", err),
   }

   tokio::signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
}