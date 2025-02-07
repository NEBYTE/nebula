<picture>
  <source media="(prefers-color-scheme: light)" srcset="https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyOWR3z3yuKt6z8SiwMQpPjTFX1mVLHuAaolWbN">
  <source media="(prefers-color-scheme: dark)" srcset="https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyOwMCPgf4f1Cjl2Pczaro3dH9SEtbyL4AKsVhF">
  <img src="https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyOWR3z3yuKt6z8SiwMQpPjTFX1mVLHuAaolWbN" alt="Nebula Logo">
</picture>

# Nebula (Early Alpha Release v0.1.0-pre.alpha.3)

[![Maintainer](https://img.shields.io/badge/maintainer-rustyspottedcatt-blue)](https://github.com/rustyspottedcatt)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-GNU_AGPLv3-blue)](https://choosealicense.com/licenses/agpl-3.0/)

> **NOTE:** Nebula is still under heavy development. Expect bugs and frequent API changes.

Nebula is a blockchain-based system that replicates ICP’s architecture—including neurons, governance, canisters, transactions, and staking. It uses **ed25519-dalek** for key management and transaction signing.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Wallet Management](#wallet-management)
  - [Transaction Processing (Direct and via Canisters)](#transaction-processing)
  - [Block Production](#block-production)
  - [Neuron Management](#neuron-management)
  - [Staking](#staking)
  - [Governance and Voting](#governance-and-voting)
- [Dependencies](#dependencies)
- [License](#license)

---

## Features

- **Wallet Management:** Generate wallets with private keys, public keys, and blockchain-compatible addresses.
- **Transaction Processing:** Build, sign, and submit transactions with dynamic fee and index calculation.
- **Consensus Engine:** Validator selection, block production, and transaction verification.
- **Governance:** Neuron-based proposals and voting.
- **Nervous System:** Neuron creation, locking/unlocking, and stake delegation.
- **Staking:** Secure staking and unstaking of tokens.
- **Canisters:** Wrap functionality (e.g., transaction submission, staking) into canisters for modularity and secure isolation.

---

## Installation

### Prerequisites

- Rust (latest stable)
- Cargo package manager

### Clone the Repository

```sh
git clone https://github.com/rustyspottedcatt/nebula.git
cd nebula
```

### Build the Project

```sh
cargo build --release
```

### Run the Application

```sh
cargo run
```

---

## Usage

### Wallet Management

Create a wallet using the built-in function. The wallet returns a private key, a public key (as a VerifyingKey), and an address.

```rust
use crate::core::api::v1::wallet::create_wallet;

let (signing_key, public_key, address) = create_wallet();
println!("Wallet created: {:x?}", address); // Shareable address
println!("Public Key: {:?}", public_key);  // Safe to share
println!("Private Key: {:?}", signing_key);  // DO NOT SHARE!
```

### Transaction Processing

#### Direct Transaction Creation

The following example shows how to build, sign, and submit a transaction directly using the consensus engine (without a canister).

```rust
use crate::core::api::v1::transaction::{build_transaction, finalize_transaction, submit_transaction};
use crate::core::types::TransactionType;

// Assume consensus_engine is already initialized and sender/receiver ledger accounts exist.
let amount = 50;
let mut tx = build_transaction(
    &mut consensus_engine,
    sender_address,    // sender's address
    receiver_address,  // receiver's address
    amount,
    0,  // memo
    0,  // nrc_memo
    TransactionType::Transfer
);
finalize_transaction(&mut tx, &signing_key)?; // Signs the transaction
submit_transaction(&mut consensus_engine, tx)?; // Submits to the mempool
```

#### Transaction Submission via Canisters

Nebula allows you to wrap functionality in a canister. In the example below, we create a canister, initialize ledger accounts for sender and receiver, build and sign a transaction, then submit it via the canister.

```rust
use crate::core::api::v1::transaction::{submit_transaction, build_transaction, finalize_transaction};
use crate::core::api::v1::wallet::create_wallet;
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use crate::core::canister::registry::CanisterRegistry;
use crate::core::consensus::{ValidatorInfo};
use crate::core::consensus::model::ConsensusEngine;
use crate::core::nervous::NervousSystem;
use crate::core::types::TransactionType;

#[tokio::main]
async fn main() {
    // Create wallet for sender.
    let (signing_key, public_key, address) = create_wallet();
    println!("Wallet created: {:x?}", address);

    // Create and register a canister.
    let mut canister_registry = CanisterRegistry::new();
    let canister_id = "my_canister".to_string();
    let canister = Canister::new(canister_id.clone(), address.clone());
    println!("Canister ID: {}", canister.canister_id);
    canister_registry.register_canister(&canister_id, canister);
    let mut registered_canister = match canister_registry.get_canister(&canister_id) {
        Some(c) => c,
        None => {
            eprintln!("Canister '{}' not found", canister_id);
            return;
        }
    };

    // Use canister-based API to submit a transaction.
    {
        // Create a wallet for the receiver.
        let (_signing_key1, public_key1, address1) = create_wallet();

        // Initialize the Nervous System and Consensus Engine.
        let nervous_system = NervousSystem::new();
        let validators = vec![ValidatorInfo { address: address.clone(), active: true }];
        let mut consensus_engine = ConsensusEngine::new(validators, nervous_system.neurons.clone());

        // Initialize ledger accounts for sender and receiver.
        consensus_engine.init_ledger(address.clone(), public_key, 100)
            .expect("Failed to initialize sender ledger");
        consensus_engine.init_ledger(address1.clone(), public_key1, 0)
            .expect("Failed to initialize receiver ledger");

        let amount = 50;
        // Build and sign the transaction.
        let mut tx = build_transaction(
            &mut consensus_engine,
            address.clone(),
            address1.clone(),
            amount,
            0,
            0,
            TransactionType::Transfer
        );
        finalize_transaction(&mut tx, &signing_key)
            .expect("Failed to sign the transaction");

        // Create a transfer payload for the canister.
        let transfer_payload = CanisterFunctionPayload::Transfer {
            consensus_engine: &mut consensus_engine,
            tx,
        };

        // Execute the transaction via the canister.
        match registered_canister.execute_function(transfer_payload) {
            Ok(msg) => println!("Successfully sent transfer: {}", msg),
            Err(err) => eprintln!("Transfer failed: {}", err),
        }
    }
}
```

### Block Production

After transactions are in the mempool, you can produce a block:

```rust
use crate::core::consensus::block::produce_block;

let block = produce_block(&mut consensus_engine, &signing_key)?;
println!("Block produced with {} transactions", block.transactions.len());
println!("Block timestamp: {}", block.header.timestamp);
```

### Neuron Management

Create a neuron for governance:

```rust
use crate::core::nervous::neuron_handler::create_neuron;

let neuron_id = create_neuron(&nervous_system, &signing_key, "Test Neuron".to_string(), 30)?;
println!("Neuron created with id: {}", neuron_id);
```

### Staking

Stake tokens to a neuron:

```rust
use crate::core::staking::staking_handler::{stake, unstake};

// init consensus_engine beforehand
let mut staking_module = core::staking::staking_module::StakingModule::new(nervous_system.neurons.clone());
stake(&mut staking_module, &mut consensus_engine, &signing_key, neuron_id, 50)?;
println!("Staked 50 tokens to neuron {}", neuron_id);
```

### Governance and Voting

Submit a proposal and vote:

```rust
use crate::core::governance::{proposal_handler::propose, voting::{vote, finalize}};

let governance = core::governance::Governance::new(nervous_system.neurons.clone());
let proposal_id = propose(&governance, "Increase block size".to_string(), &signing_key, neuron_id)?;
println!("Proposal created with id: {}", proposal_id);

match vote(&governance, &signing_key, neuron_id, proposal_id, true, 10) {
    Ok(_) => println!("Voted on proposal {}", proposal_id),
    Err(e) => println!("Voting failed: {}", e),
}

let proposal_result = finalize(&governance, proposal_id)?;
println!("Proposal finalized with result: {}", proposal_result);
```

---

## Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "full"] }
bincode = "1.3"
serde = { version = "1.0", features = ["derive"] }
ed25519-dalek = { version = "2", features = ["rand_core", "serde"] }
rand = "0.8"
chrono = { version = "0.4", features = ["serde"] }
sha2 = "0.10"
hex = "0.4"
```

---

## License

Distributed under the [GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/) license.