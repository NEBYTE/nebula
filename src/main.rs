/// Entry point for Nebula.
/// Launches P2P (using QUIC), consensus, staking, and storage.

mod consensus;
mod staking;
mod governance;
mod api;
mod types;

mod crypto;

use std::error::Error;
use std::sync::Arc;
use std::net::SocketAddr;
use consensus::{ConsensusEngine, ValidatorInfo};
use staking::{StakingModule, StakingAccount};
use governance::Governance;
use api::{create_wallet, build_transaction};
use sled::Db;
use rand::rngs::OsRng;
use ed25519_dalek::{SigningKey};
use rcgen::generate_simple_self_signed;
use quinn::{Endpoint, TransportConfig};
use tokio_rustls::rustls::{Certificate, PrivateKey};
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Generate a self-signed certificate
    let cert = generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();

    // Configure the QUIC server with the self-signed certificate
    let cert_chain = vec![Certificate(cert_der.clone())];
    let private_key = PrivateKey(priv_key);

    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(
        tokio_rustls::rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, private_key)?,
    ));

    // Set transport configurations
    let mut transport_config = TransportConfig::default();
    transport_config.max_concurrent_bidi_streams(100u32.into());
    server_config.transport = Arc::new(transport_config);

    // Bind the QUIC endpoint
    let addr: SocketAddr = "127.0.0.1:4433".parse()?;
    let endpoint = Endpoint::server(server_config, addr)?;
    println!("Listening on {}", addr);

    spawn(async move {
        while let Some(connecting) = endpoint.accept().await {
            match connecting.await {
                Ok(connection) => {
                    println!("New connection from {}", connection.remote_address());
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    });

    // Initialize the sled database
    let db: Db = sled::open("nebula_db")?;

    // Generate a signing key and corresponding verifying key
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let public_bytes = verifying_key.to_bytes();

    // Define validators for the consensus engine
    let validators = vec![ValidatorInfo {
        address: public_bytes,
        stake: 1000,
        active: true,
    }];

    // Initialize the consensus engine
    let mut consensus_engine = ConsensusEngine::new(validators, db.clone());

    // Create a staking module
    let mut staking_module = StakingModule::new();
    staking_module.accounts.push(StakingAccount {
        address: public_bytes,
        balance: 10000,
        staked: 1000,
    });

    // Initialize the governance module
    let mut governance = Governance::new();

    // Create a wallet and add it to the staking module
    let wallet = create_wallet();
    staking_module.accounts.push(StakingAccount {
        address: wallet,
        balance: 500,
        staked: 0,
    });

    // Stake funds and distribute rewards
    staking_module.stake(wallet, 100)?;
    staking_module.distribute_rewards(50);

    // Select the next validator
    if let Some(next_val) = consensus_engine.select_next_validator() {
        println!("Next validator: {:?}", next_val);
        consensus_engine.slash(next_val, 10);
    }

    // Build a transaction and produce a block
    let tx = build_transaction(wallet, public_bytes, 10, 0);
    let block = consensus_engine.produce_block(&signing_key, vec![tx]);
    match consensus_engine.validate_block(&block) {
        Ok(_) => println!("Block valid."),
        Err(e) => println!("Block invalid: {}", e),
    }

    // Save the latest block in the database
    let encoded_block = bincode::serialize(&block)?;
    db.insert("latest_block", encoded_block)?;
    db.flush()?;

    // Create and vote on a governance proposal
    let proposal_id = governance.propose("Increase Gas Limit".into(), public_bytes);
    governance.vote(proposal_id, true, 50)?;
    if let Some(passed) = governance.finalize(proposal_id) {
        println!("Proposal passed? {}", passed);
    }

    // Print account balances and staked amounts
    for acc in &staking_module.accounts {
        println!(
            "Account: {:?}, balance: {}, staked: {}",
            acc.address, acc.balance, acc.staked
        );
    }

    Ok(())
}
