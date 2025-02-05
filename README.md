# Nebula

Nebula is a blockchain-based system that replicates ICP's architecture, including neurons, governance, canisters, transactions, and staking. It uses ed25519-dalek cryptography for key management, signing transactions, and securing governance operations.

## Features
- **Wallet Management:** Generates private keys, public keys, and blockchain-compatible addresses.
- **Transaction Processing:** Creates, signs, and submits transactions with dynamic fee and index calculation.
- **Consensus Engine:** Selects validators, produces blocks, and verifies transactions.
- **Governance System:** Proposals, voting, and neuron-based decision-making.
- **Nervous System:** Manages neuron creation, locking/unlocking, and stake delegation.
- **Staking Module:** Allows users to stake and unstake tokens.

## Testing main.rs

Expected output:
```rust
Running `target\debug\nebulacrypto.exe`
Wallet created: "aad59a0c37498e7ba01c4db496205c568dcd32b5aa8345cbd819ef6b4bb9633e"
Block produced with 1 transaction(s)
Block timestamp: 1738755575
Neuron created with id: 1
Staked 50 tokens to neuron 1
Unstaking failed: Neuron is locked in dissolve delay
Proposal created with id: 1
Voted on proposal 1
Proposal finalized with result: true

Process finished with exit code 0
```

## Installation
### Prerequisites
- Rust (latest stable version)
- Nightly
- Cargo package manager

### Clone the Repository
```sh
$ git clone https://github.com/rustyspottedcatt/nebula
$ cd NebulaCrypto
```

### Install Dependencies
```sh
$ cargo build
```

### Run the Application
```sh
$ cargo run
```

## Usage
### Creating a Wallet
```rust
let (signing_key, public_key, address) = api::create_wallet();
println!("Address: {}", address); // Shareable
println!("Public Key: {}", public_key); // Shareable -- Public key will be used for identification in the future
println!("Private Key: {}", signing_key); // Do not SHARE
```

### Creating and Signing Transactions
```rust
let recipient = "recipient_address";
let amount = 100;

let mut tx = api::build_transaction(&mut consensus_engine, address, recipient, amount);
api::finalize_transaction(&mut tx)?;
api::submit_transaction(&mut consensus_engine, tx)?;
```

### Producing a Block
```rust
let block = consensus_engine.produce_block(&signing_key)?;
println!("Block produced with {} transaction(s)", block.transactions.len());
println!("Block timestamp: {}", block.header.timestamp);
```

### Neuron Management
```rust
let neuron_id = nervous_system.create_neuron(&signing_key, "Test Neuron".to_string(), 30)?;
println!("Neuron created with id: {}", neuron_id);
```

### Staking
```rust
let mut staking_module = staking::StakingModule::new(nervous_system.neurons.clone());
staking_module.stake(&signing_key, neuron_id, 50)?;
println!("Staked 50 tokens to neuron {}", neuron_id);
```

### Governance and Voting
```rust
let governance = governance::Governance::new(nervous_system.neurons.clone());

let proposal_id = governance.propose("Increase block size".to_string(), &signing_key, neuron_id)?;
println!("Proposal created with id: {}", proposal_id);

match governance.vote(&signing_key, neuron_id, proposal_id, true, 10) {
Ok(_) => println!("Voted on proposal {}", proposal_id),
Err(e) => println!("Voting failed: {}", e),
}

let proposal_result = governance.finalize(proposal_id)?;
println!("Proposal finalized with result: {}", proposal_result);
```

## Dependencies
- tokio = { version = "1", features = ["rt-multi-thread", "macros", "full"] }
- bincode = "1.3"
- serde = { version = "1.0", features = ["derive"] }
- ed25519-dalek = { version = "2", features = ["rand_core", "serde"]}
- rand = "0.8"
- chrono = { version = "0.4.39", features = ["serde"]}
- sha2 = "0.10.8"
- hex = "0.4.3"

## License
````markdown
Creative Commons Legal Code

CC0 1.0 Universal

    CREATIVE COMMONS CORPORATION IS NOT A LAW FIRM AND DOES NOT PROVIDE
    LEGAL SERVICES. DISTRIBUTION OF THIS DOCUMENT DOES NOT CREATE AN
    ATTORNEY-CLIENT RELATIONSHIP. CREATIVE COMMONS PROVIDES THIS
    INFORMATION ON AN "AS-IS" BASIS. CREATIVE COMMONS MAKES NO WARRANTIES
    REGARDING THE USE OF THIS DOCUMENT OR THE INFORMATION OR WORKS
    PROVIDED HEREUNDER, AND DISCLAIMS LIABILITY FOR DAMAGES RESULTING FROM
    THE USE OF THIS DOCUMENT OR THE INFORMATION OR WORKS PROVIDED
    HEREUNDER.

Statement of Purpose

The laws of most jurisdictions throughout the world automatically confer
exclusive Copyright and Related Rights (defined below) upon the creator
and subsequent owner(s) (each and all, an "owner") of an original work of
authorship and/or a database (each, a "Work").

Certain owners wish to permanently relinquish those rights to a Work for
the purpose of contributing to a commons of creative, cultural and
scientific works ("Commons") that the public can reliably and without fear
of later claims of infringement build upon, modify, incorporate in other
works, reuse and redistribute as freely as possible in any form whatsoever
and for any purposes, including without limitation commercial purposes.
These owners may contribute to the Commons to promote the ideal of a free
culture and the further production of creative, cultural and scientific
works, or to gain reputation or greater distribution for their Work in
part through the use and efforts of others.

For these and/or other purposes and motivations, and without any
expectation of additional consideration or compensation, the person
associating CC0 with a Work (the "Affirmer"), to the extent that he or she
is an owner of Copyright and Related Rights in the Work, voluntarily
elects to apply CC0 to the Work and publicly distribute the Work under its
terms, with knowledge of his or her Copyright and Related Rights in the
Work and the meaning and intended legal effect of CC0 on those rights.

1. Copyright and Related Rights. A Work made available under CC0 may be
   protected by copyright and related or neighboring rights ("Copyright and
   Related Rights"). Copyright and Related Rights include, but are not
   limited to, the following:

   i. the right to reproduce, adapt, distribute, perform, display,
   communicate, and translate a Work;
   ii. moral rights retained by the original author(s) and/or performer(s);
   iii. publicity and privacy rights pertaining to a person's image or
   likeness depicted in a Work;
   iv. rights protecting against unfair competition in regards to a Work,
   subject to the limitations in paragraph 4(a), below;
   v. rights protecting the extraction, dissemination, use and reuse of data
   in a Work;
   vi. database rights (such as those arising under Directive 96/9/EC of the
   European Parliament and of the Council of 11 March 1996 on the legal
   protection of databases, and under any national implementation
   thereof, including any amended or successor version of such
   directive); and
   vii. other similar, equivalent or corresponding rights throughout the
   world based on applicable law or treaty, and any national
   implementations thereof.

2. Waiver. To the greatest extent permitted by, but not in contravention
   of, applicable law, Affirmer hereby overtly, fully, permanently,
   irrevocably and unconditionally waives, abandons, and surrenders all of
   Affirmer's Copyright and Related Rights and associated claims and causes
   of action, whether now known or unknown (including existing as well as
   future claims and causes of action), in the Work (i) in all territories
   worldwide, (ii) for the maximum duration provided by applicable law or
   treaty (including future time extensions), (iii) in any current or future
   medium and for any number of copies, and (iv) for any purpose whatsoever,
   including without limitation commercial, advertising or promotional
   purposes (the "Waiver"). Affirmer makes the Waiver for the benefit of each
   member of the public at large and to the detriment of Affirmer's heirs and
   successors, fully intending that such Waiver shall not be subject to
   revocation, rescission, cancellation, termination, or any other legal or
   equitable action to disrupt the quiet enjoyment of the Work by the public
   as contemplated by Affirmer's express Statement of Purpose.

3. Public License Fallback. Should any part of the Waiver for any reason
   be judged legally invalid or ineffective under applicable law, then the
   Waiver shall be preserved to the maximum extent permitted taking into
   account Affirmer's express Statement of Purpose. In addition, to the
   extent the Waiver is so judged Affirmer hereby grants to each affected
   person a royalty-free, non transferable, non sublicensable, non exclusive,
   irrevocable and unconditional license to exercise Affirmer's Copyright and
   Related Rights in the Work (i) in all territories worldwide, (ii) for the
   maximum duration provided by applicable law or treaty (including future
   time extensions), (iii) in any current or future medium and for any number
   of copies, and (iv) for any purpose whatsoever, including without
   limitation commercial, advertising or promotional purposes (the
   "License"). The License shall be deemed effective as of the date CC0 was
   applied by Affirmer to the Work. Should any part of the License for any
   reason be judged legally invalid or ineffective under applicable law, such
   partial invalidity or ineffectiveness shall not invalidate the remainder
   of the License, and in such case Affirmer hereby affirms that he or she
   will not (i) exercise any of his or her remaining Copyright and Related
   Rights in the Work or (ii) assert any associated claims and causes of
   action with respect to the Work, in either case contrary to Affirmer's
   express Statement of Purpose.

4. Limitations and Disclaimers.

   a. No trademark or patent rights held by Affirmer are waived, abandoned,
   surrendered, licensed or otherwise affected by this document.
   b. Affirmer offers the Work as-is and makes no representations or
   warranties of any kind concerning the Work, express, implied,
   statutory or otherwise, including without limitation warranties of
   title, merchantability, fitness for a particular purpose, non
   infringement, or the absence of latent or other defects, accuracy, or
   the present or absence of errors, whether or not discoverable, all to
   the greatest extent permissible under applicable law.
   c. Affirmer disclaims responsibility for clearing rights of other persons
   that may apply to the Work or any use thereof, including without
   limitation any person's Copyright and Related Rights in the Work.
   Further, Affirmer disclaims responsibility for obtaining any necessary
   consents, permissions or other rights required for any use of the
   Work.
   d. Affirmer understands and acknowledges that Creative Commons is not a
   party to this document and has no duty or obligation with respect to
   this CC0 or use of the Work.
````