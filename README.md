<picture>
  <source media="(prefers-color-scheme: light)" srcset="https://example.com/light.png">
  <source media="(prefers-color-scheme: dark)" srcset="https://example.com/dark.png">
  <img src="https://example.com/light.png" alt="Nebula Logo">
</picture>

# Nebula (Early Alpha Release v1.0.0)

[![Maintainer](https://img.shields.io/badge/maintainer-NEBYTE-blue)](https://github.com/rustyspottedcatt)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-GNU_AGPLv3-blue)](https://choosealicense.com/licenses/agpl-3.0/)

Nebula is a blockchain-based system featuring wallet management, transaction processing, a consensus engine with staking, neuron-based governance, modular canister execution, and dynamic node registration. It is a personal, non-commercial learning project inspired by ICP's architecture and is not intended for production use.

## Table of Contents

- [Features](#features)
- [Whitepaper](#whitepaper)
- [Installation](#installation)
- [Configuration](#configuration)
  - [config.toml](#configtoml)
- [Usage](#usage)
  - [Wallet Management](#wallet-management)
  - [Transaction Processing](#transaction-processing)
  - [Canister Transactions](#canister-transactions)
  - [Block Production](#block-production)
  - [Neuron Management](#neuron-management)
  - [Staking](#staking)
  - [Governance and Voting](#governance-and-voting)
  - [Node Registry](#node-registry)
- [Dependencies](#dependencies)
- [License](#license)

## Features

- **Wallet Management**: Create and manage wallets with secure keys.
- **Transaction Processing**: Build, sign, and submit transactions.
- **Consensus Engine**: Validator selection, block production, and ledger management.
- **Governance**: Neuron-based proposals and weighted voting.
- **Nervous System**: Neuron creation, locking, and stake delegation.
- **Staking**: Secure staking and unstaking of tokens via canisters.
- **Canisters**: Modular execution of on-chain functions.
- **Node Registry**: Dynamic node registration and configuration via a registry.

## Whitepaper

[Whitepaper Link](https://whitepapersonline.com/en/whitepaper/nebula-a-decentralized-open-source-blockchain-for-enhanced-governance)

[Nebula - Whitepaper.pdf](https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyO0uPtcuIgO5dR7MSuQXNazqoA6bVipTyxmHfC)

## Installation

### Prerequisites

- Rust (latest stable) + Nightly
- Cargo

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

For example, to run a node called node1:

```sh
cargo run node1
```

## Configuration

Nebula uses a configuration file (`config.toml`) to set parameters for nodes and network settings.

### config.toml

```toml
[node1]
name = "NebulaNode1"
port = 30333
initial_balance = 1000
db_path = "data/nodes/nebula_storage_node1"
data_center_owner = "Owner A"
fiber_state = "Operational"
location = "Location A"
node_provider = "Provider A"
status = "active"
node_provider_id = "provider-id-001"
node_operator_id = "operator-id-001"
subnet_id = "subnet-001"
ip_address = "127.0.0.1"
```

### Example Production Usage:

```toml
[node1]
name = "NebulaNode1"
port = 30333
initial_balance = 1000
db_path = "data/nodes/nebula_storage_node1"
data_center_owner = "Owner A"
fiber_state = "Operational"
location = "Location A"
node_provider = "Provider A"
status = "active"
node_provider_id = "provider-id-001"
node_operator_id = "operator-id-001"
subnet_id = "subnet-001"
ip_address = "127.0.0.1"

[node2]
name = "NebulaNode2"
port = 30334
initial_balance = 1000
db_path = "data/nodes/nebula_storage_node2"
data_center_owner = "Owner B"
fiber_state = "Operational"
location = "Location B"
node_provider = "Provider B"
status = "active"
node_provider_id = "provider-id-002"
node_operator_id = "operator-id-002"
subnet_id = "subnet-002"
ip_address = "127.0.0.1"

[network]
bootstrap_nodes = ["127.0.0.1:30333", "127.0.0.1:30334"]
```

## Usage

### Wallet Management

```rust
use crate::core::api::v1::wallet::create_wallet;
let wallet = create_wallet(db.clone());
println!("Wallet created: {}", wallet.address);
```

### Transaction Processing (Using Canisters)

```rust
use crate::core::canister::canister::{Canister, CanisterFunctionPayload};
let mut canister = Canister::new("transaction_canister".to_string(), sender_address.clone(), db.clone());
let result = canister.execute_function(CanisterFunctionPayload::Transfer { amount: 50, sender: sender_address, receiver: receiver_address });
println!("Transaction result: {:?}", result);
```

### Staking (Using Canisters)

```rust
let mut canister = Canister::new("staking_canister".to_string(), sender_address.clone(), db.clone());
let result = canister.execute_function(CanisterFunctionPayload::Stake { neuron_id: 12345, amount: 500 });
println!("Staking Result: {:?}", result);
```

### Unstaking (Using Canisters)

```rust
let result = canister.execute_function(CanisterFunctionPayload::Unstake { neuron_id: 12345, amount: 200 });
println!("Unstaking Result: {:?}", result);
```

### Node Registry

```rust
use crate::core::node::{NodeRegistry, Node};
let node_registry = NodeRegistry::new(db.clone());
let node = Node {
    data_center_owner: "Owner A".to_string(),
    fiber_state: "Operational".to_string(),
    dc_id: "node1".to_string(),
    location: "Location A".to_string(),
    node_provider: "Provider A".to_string(),
    status: "active".to_string(),
    node_provider_id: "provider-id-001".to_string(),
    node_operator_id: "operator-id-001".to_string(),
    subnet_id: "subnet-001".to_string(),
    ip_address: "127.0.0.1".to_string(),
};
node_registry.register_node(node);
```

## License

Distributed under the [GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/) license.