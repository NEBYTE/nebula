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

Nebula is a blockchain-based system that replicates ICP’s architecture—including neurons, governance, canisters, transactions, and staking. It utilizes **ed25519-dalek** for key management and transaction signing.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Wallet Management](#wallet-management)
  - [Transaction Processing](#transaction-processing)
  - [Canister Transactions](#canister-transactions)
  - [Block Production](#block-production)
  - [Neuron Management](#neuron-management)
  - [Staking](#staking)
  - [Governance and Voting](#governance-and-voting)
- [Dependencies](#dependencies)
- [License](#license)

---

## Features

- **Wallet Management** - Create and manage wallets with private/public keys and blockchain-compatible addresses.
- **Transaction Processing** - Build, sign, and submit transactions securely.
- **Consensus Engine** - Validator selection, block production, and transaction verification.
- **Governance** - Neuron-based proposals and voting.
- **Nervous System** - Neuron creation, locking/unlocking, and stake delegation.
- **Staking** - Secure staking and unstaking of tokens.
- **Canisters** - Modular execution of functions via on-chain canisters.

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

```rust
use crate::core::api::v1::wallet::create_wallet;

// Create a new wallet
let (signing_key, public_key, sender_address) = create_wallet();
println!("Sender Wallet created: {:x?}", sender_address);

let (_receiver_signing, receiver_public, receiver_address) = create_wallet();
println!("Receiver Wallet created: {:x?}", receiver_address);
```

---

### Transaction Processing

#### Direct Transaction

```rust
use crate::core::api::v1::transaction::{build_transaction, finalize_transaction};
use crate::core::types::TransactionType;

let mut tx = build_transaction(
    &mut consensus_engine,
    sender_address.clone(),
    receiver_address.clone(),
    50,  // Amount
    0,   // Memo
    0,   // NRC Memo
    TransactionType::Transfer,
); 

// above assumes ledger were made with the sufficient balance for the sender address.

finalize_transaction(&mut tx, &signing_key).expect("Failed to finalize transaction");
```

---

### Canister Transactions

```rust
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
use crate::core::canister::registry::CanisterRegistry;

// Register a canister
let mut canister_registry = CanisterRegistry::new();
let canister = Canister::new("my_canister".to_string(), sender_address.clone());
println!("Canister created with ID: {}", canister.canister_id);

canister_registry.register_canister(&canister.canister_id, canister.clone());
```

---

### Block Production

```rust
use crate::core::consensus::consensus::run_consensus_loop;
use std::time::Duration;

let target_cycle = Duration::from_secs(1 / 2); // 0.5s
let signing_key_clone = signing_key.clone();
let mut consensus_engine_clone = consensus_engine.clone(); // consensus_engine uses Arc<Mutex<T>>, everything is synchronized.

tokio::spawn(async move {
    run_consensus_loop(
        &mut consensus_engine_clone,
        &signing_key_clone,
        target_cycle,
    ).await;
});
println!("🚀 Blockchain node is running! Listening for transactions...");
```

---

### Neuron Management

```rust
use crate::core::nervous::{create_neuron, NervousSystem};

let mut nervous_system = NervousSystem::new();
let neuron_id = create_neuron(&mut nervous_system, &signing_key, "John Doe".to_string(), 365)
    .expect("Failed to create neuron");
println!("Created Neuron with ID: {}", neuron_id);
```

---

### Staking

```rust
use crate::core::staking::{stake, StakingModule};

let mut staking_module = StakingModule::new(nervous_system.neurons.clone());
stake(&mut staking_module, &mut consensus_engine, &signing_key, neuron_id, 500)
    .expect("Failed to stake 500 tokens");
println!("Staked 500 tokens to neuron {}", neuron_id);
```

---

### Governance and Voting

```rust
use crate::core::governance::Governance;

let mut governance_module = Governance::new(nervous_system.neurons.clone());
```

---

## Dependencies

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

