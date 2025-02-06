<picture>
  <source media="(prefers-color-scheme: light)" srcset="https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyOWR3z3yuKt6z8SiwMQpPjTFX1mVLHuAaolWbN">
  <source media="(prefers-color-scheme: dark)" srcset="https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyOwMCPgf4f1Cjl2Pczaro3dH9SEtbyL4AKsVhF">
  <img src="https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyOWR3z3yuKt6z8SiwMQpPjTFX1mVLHuAaolWbN" alt="Light Image">
</picture>

---

# Nebula (Early Alpha Release v0.1.0-pre.alpha.1)

![Maintainer](https://img.shields.io/badge/maintainer-rustyspottedcatt-blue)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-GNU_AGPLv3-blue)](https://choosealicense.com/licenses/agpl-3.0/)

> [!NOTE]
> Nebula is still heavily in work in progress, expect bugs!
> 
**Nebula** is a blockchain-based system that replicates ICP's architecture, including neurons, governance, canisters, transactions, and staking. It uses **ed25519-dalek** cryptography for key management, transaction signing, and governance security.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Dependencies](#dependencies)
- [License](#license)

---

## Features

- Wallet Management: Generates private keys, public keys, and blockchain-compatible addresses.
- Transaction Processing: Creates, signs, and submits transactions with dynamic fee and index calculation.
- Consensus Engine: Selects validators, produces blocks, and verifies transactions.
- Governance System: Proposals, voting, and neuron-based decision-making.
- Nervous System: Manages neuron creation, locking/unlocking, and stake delegation.
- Staking Module: Allows users to stake and unstake tokens securely.

---

## Installation

### Prerequisites

Ensure you have the following installed:
- Rust (latest stable version)
- Rust Nightly
- Cargo package manager

### Clone the Repository
```sh
$ git clone https://github.com/rustyspottedcatt/nebula
$ cd nebula
```

### Build the Project
```sh
$ cargo build --release
```

### Run the Application
```sh
$ cargo run
```

---

## Usage

### Creating a Wallet
```rust
use core::api::v1::*;

let (signing_key, _public_key, address) = create_wallet();
println!("Wallet created: {:x?}", address); // Shareable (used to exchange)
println!("Public Key: {}", public_key); // Shareable
println!("Private Key: {}", signing_key); // Do not SHARE
```

### Creating and Signing Transactions
```rust
use core::api::v1::*;

let recipient = "recipient_address".to_string();
let amount = 100;

let mut tx = build_transaction(&mut consensus_engine, address, recipient, amount);
finalize_transaction(&mut tx)?;
submit_transaction(&mut consensus_engine, tx)?;
```

### Producing a Block
```rust
use crate::core::consensus::block::produce_block;

let block = produce_block(&mut consensus_engine, &signing_key)?;
println!("Block produced with {} transaction(s)", block.transactions.len());
println!("Block timestamp: {}", block.header.timestamp);
```

### Neuron Management
```rust
use crate::core::nervous::neuron_handler::create_neuron;

let neuron_id = create_neuron(&nervous_system, &signing_key, "Test Neuron".to_string(), 30)?;
println!("Neuron created with id: {}", neuron_id);
```

### Staking
```rust
use crate::core::staking::staking_handler::{stake, unstake};

let mut staking_module = core::staking::staking_module::StakingModule::new(nervous_system.neurons.clone());
stake(&mut staking_module, &signing_key, neuron_id, 50)?;
println!("Staked 50 tokens to neuron {}", neuron_id);
```

### Governance and Voting
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

```toml
tokio = { version = "1", features = ["rt-multi-thread", "macros", "full"] }
bincode = "1.3"
serde = { version = "1.0", features = ["derive"] }
ed25519-dalek = { version = "2", features = ["rand_core", "serde"] }
rand = "0.8"
chrono = { version = "0.4.39", features = ["serde"] }
sha2 = "0.10.8"
hex = "0.4.3"
```

---

## License

Distributed under the [GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/) license.
