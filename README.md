# Nebula

Nebula is a blockchain-based application built using Rust. It implements consensus mechanisms, staking, governance, and cryptographic operations using state-of-the-art libraries like `ed25519-dalek`, `quinn`, and `sled`.

---

## Features

- **Consensus**: Implements a validator-based consensus mechanism for block production and validation.
- **Staking**: Allows users to stake their tokens, earn rewards, and participate in validator selection.
- **Governance**: Supports decentralized governance proposals and voting mechanisms.
- **Cryptographic Security**: Uses Ed25519 digital signatures for transaction signing and block validation.
- **Networking**: Utilizes QUIC for efficient and secure peer-to-peer communication.
- **Persistent Storage**: Built on `sled`, a modern embedded database for data persistence.

---

## Project Structure

```plaintext
src/
├── main.rs          # Entry point for the application
├── crypto.rs        # Cryptographic operations (signing, verification)
├── consensus.rs     # Consensus engine and block validation logic
├── staking.rs       # Staking module for validators and rewards
├── governance.rs    # Governance module for proposals and voting
├── api.rs           # API for wallets, transactions, and other utilities
├── types.rs         # Shared types like Block, Transaction, Address, etc.
```

---

## Getting Started

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) (edition 2024)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust package manager)

---

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/rustyspottedcatt/nebula.git
   cd nebula
   ```

2. Build the project:
   ```sh
   cargo build
   ```

3. Run the project:
   ```sh
   cargo run
   ```

---

## Usage

### Key Features

1. **Consensus Engine**:
    - Produces and validates blocks.
    - Selects the next validator based on stake and activity.

2. **Staking**:
    - Stake tokens to become a validator.
    - Earn rewards proportional to your stake.

3. **Governance**:
    - Propose changes to the blockchain (e.g., gas limit increases).
    - Vote on proposals to influence blockchain decisions.

4. **Cryptographic Operations**:
    - Sign and verify transactions using Ed25519 signatures.
