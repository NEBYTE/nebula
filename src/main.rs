pub mod core;

use config::Config;
use rocksdb::DB;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::{io, task};

use crate::core::api::v1::wallet::create_wallet;
use crate::core::nervous::{create_neuron, NervousSystem};
use crate::core::staking::{stake, StakingModule};
use crate::core::consensus::model::ConsensusEngine;
use crate::core::consensus::consensus::run_consensus_loop;
use crate::core::consensus::validator::{build_validator, wrap_validator};
use crate::core::network::machine::{Node, NodeRegistry};

#[tokio::main]
async fn main() {
   println!("⚡ Loading configuration...");
   let settings = Config::builder()
       .add_source(config::File::with_name("config.toml"))
       .build()
       .expect("Failed to load config");

   let args: Vec<String> = std::env::args().collect();
   let node_key = if args.len() > 1 {
      args[1].clone()
   } else {
      "node1".to_string()
   };

   let node_name: String = settings.get(&format!("{}.name", node_key)).unwrap_or_else(|_| "NebulaNode".to_string());
   let network_port: u16 = settings.get(&format!("{}.port", node_key)).unwrap_or(30333);
   let initial_balance: u64 = settings.get(&format!("{}.initial_balance", node_key)).unwrap_or(1000);
   let db_path: String = settings.get(&format!("{}.db_path", node_key)).unwrap_or_else(|_| "nebula_storage".to_string());
   let peer_addresses: Vec<String> = settings.get("network.bootstrap_nodes").unwrap_or_else(|_| vec![]);

   println!("🚀 Starting {} on port {} with initial balance {}", node_name, network_port, initial_balance);

   println!("⚡ Initializing RocksDB at {}...", db_path);
   let db = Arc::new(DB::open_default(db_path).expect("Failed to open RocksDB"));

   let node_registry = NodeRegistry::new(Arc::clone(&db));
   let node = Node {
      data_center_owner: settings.get(&format!("{}.data_center_owner", node_key)).unwrap(),
      fiber_state: settings.get(&format!("{}.fiber_state", node_key)).unwrap(),
      dc_id: node_key.clone(),
      location: settings.get(&format!("{}.location", node_key)).unwrap(),
      node_provider: settings.get(&format!("{}.node_provider", node_key)).unwrap(),
      status: settings.get(&format!("{}.status", node_key)).unwrap(),
      node_provider_id: settings.get(&format!("{}.node_provider_id", node_key)).unwrap(),
      node_operator_id: settings.get(&format!("{}.node_operator_id", node_key)).unwrap(),
      subnet_id: settings.get(&format!("{}.subnet_id", node_key)).unwrap(),
      ip_address: settings.get(&format!("{}.ip_address", node_key)).unwrap(),
   };
   node_registry.register_node(node);

   println!("⚡ Creating wallet...");
   let wallet = create_wallet(Arc::clone(&db));
   println!("✅ Wallet created with address: {}", wallet.address);

   println!("⚡ Initializing Nervous System...");
   let mut nervous_system = NervousSystem::new(Arc::clone(&db));
   println!("✅ Nervous System initialized.");

   println!("⚡ Initializing Staking Module...");
   let mut staking_module = StakingModule::new(Arc::clone(&nervous_system.neurons), Arc::clone(&db));
   println!("✅ Staking Module initialized.");

   println!("⚡ Creating Neuron...");
   let neuron_id = create_neuron(&mut nervous_system, &wallet.signing_key, "LiveNeuron".to_string(), 365)
       .expect("Failed to create neuron");
   println!("✅ Neuron created with ID: {}", neuron_id);

   println!("⚡ Building Validator...");
   let mut validator = build_validator(&mut nervous_system, neuron_id)
       .expect("Failed to build validator");
   validator.active = true;
   println!("✅ Validator built successfully.");

   println!("⚡ Wrapping Validator...");
   let validators = wrap_validator(validator);
   println!("✅ Validators wrapped: {:?}", validators.lock());

   println!("⚡ Initializing Consensus Engine...");
   let mut consensus_engine = ConsensusEngine::new(validators, nervous_system.neurons.clone(), Arc::clone(&db));
   println!("✅ Consensus Engine initialized with validators: {:?}", consensus_engine.validators.lock());

   println!("⚡ Initializing ledger...");
   consensus_engine.init_ledger(wallet.address.clone(), wallet.public_key, initial_balance);
   println!("✅ Ledger initialized.");

   println!("⚡ Staking tokens...");
   stake(
      &mut nervous_system,
      &mut staking_module,
      &mut consensus_engine,
      &wallet.signing_key,
      neuron_id,
      500,
   )
       .expect("Failed to stake tokens");
   println!("✅ Staking complete.");

   let mut consensus_engine_clone = consensus_engine.clone();
   let mut staking_module_clone = staking_module.clone();
   let mut nervous_system_clone = nervous_system.clone();
   let signing_key_clone = wallet.signing_key.clone();

   println!("⚡ Binding TCP Listener on port {}...", network_port);
   let listener = TcpListener::bind(("0.0.0.0", network_port))
       .await
       .expect("Failed to bind TCP listener");
   println!("✅ Listening for peer connections on port {}", network_port);

   tokio::spawn(async move {
      println!("⚡ Starting consensus loop...");
      run_consensus_loop(&mut nervous_system_clone, &mut consensus_engine_clone, &mut staking_module_clone, &signing_key_clone).await;
      println!("✅ Consensus loop started.");
   });

   let peers = Arc::new(Mutex::new(HashMap::new()));

   println!("⚡ Connecting to peers...");
   for peer in peer_addresses {
      if let Ok(stream) = TcpStream::connect(peer.clone()).await {
         println!("✅ Connected to peer {}", peer);
         let peers_clone = Arc::clone(&peers);
         tokio::spawn(async move {
            let _ = handle_connection(stream, peers_clone).await;
         });
      } else {
         println!("❌ Failed to connect to peer {}", peer);
      }
   }

   loop {
      match listener.accept().await {
         Ok((stream, addr)) => {
            println!("🔗 New connection from {}", addr);
            let peers_clone = Arc::clone(&peers);
            task::spawn(async move {
               if let Err(e) = handle_connection(stream, peers_clone).await {
                  eprintln!("❌ Error handling connection from {}: {}", addr, e);
               }
            });
         }
         Err(e) => eprintln!("❌ Failed to accept connection: {}", e),
      }
   }
}

async fn handle_connection(
   stream: TcpStream,
   peers: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>,
) -> io::Result<()> {
   let addr = stream.peer_addr()?.to_string();
   let stream = Arc::new(Mutex::new(stream));
   {
      let mut peers_map = peers.lock().await;
      peers_map.insert(addr.clone(), Arc::clone(&stream));
   }
   println!("🔗 Peer connected: {}", addr);
   let mut buffer = vec![0u8; 1024];
   loop {
      let n = {
         let mut locked_stream = stream.lock().await;
         match locked_stream.read(&mut buffer).await {
            Ok(n) => n,
            Err(e) => {
               eprintln!("❌ Error reading from peer {}: {}", addr, e);
               break;
            }
         }
      };
      if n == 0 {
         println!("🔌 Peer {} disconnected", addr);
         break;
      }
      let message = String::from_utf8_lossy(&buffer[..n]);
      println!("📩 Message from {}: {}", addr, message);
      {
         let mut locked_stream = stream.lock().await;
         if let Err(e) = locked_stream.write_all(b"ACK").await {
            eprintln!("❌ Error writing to peer {}: {}", addr, e);
            break;
         }
      }
   }
   {
      let mut peers_map = peers.lock().await;
      peers_map.remove(&addr);
   }
   println!("🔌 Connection closed with peer: {}", addr);
   Ok(())
}
