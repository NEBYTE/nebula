use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rocksdb::DB;
use bincode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub data_center_owner: String,
    pub fiber_state: String,
    pub dc_id: String,
    pub location: String,
    pub node_provider: String,
    pub status: String,
    pub node_provider_id: String,
    pub node_operator_id: String,
    pub subnet_id: String,
    pub ip_address: String,
}

#[derive(Clone)]
pub struct NodeRegistry {
    nodes: Arc<Mutex<HashMap<String, Node>>>,
    db: Arc<DB>,
}

impl NodeRegistry {
    pub fn new(db: Arc<DB>) -> Self {
        let registry = Self {
            nodes: Arc::new(Mutex::new(HashMap::new())),
            db,
        };
        registry.load_state();
        registry
    }

    pub fn register_node(&self, node: Node) {
        {
            let mut nodes = self.nodes.lock().unwrap();
            nodes.insert(node.dc_id.clone(), node.clone());
        }
        let serialized = bincode::serialize(&node).unwrap();
        let key = format!("node_{}", node.dc_id);
        self.db.put(key.as_bytes(), serialized).unwrap();
    }

    pub fn get_node(&self, id: &str) -> Option<Node> {
        let nodes = self.nodes.lock().unwrap();
        nodes.get(id).cloned()
    }

    pub fn get_all_nodes(&self) -> Vec<Node> {
        let nodes = self.nodes.lock().unwrap();
        nodes.values().cloned().collect()
    }

    pub fn update_node(&self, node: Node) {
        {
            let mut nodes = self.nodes.lock().unwrap();
            nodes.insert(node.dc_id.clone(), node.clone());
        }
        let serialized = bincode::serialize(&node).unwrap();
        let key = format!("node_{}", node.dc_id);
        self.db.put(key.as_bytes(), serialized).unwrap();
    }

    pub fn load_state(&self) {
        let mut nodes = self.nodes.lock().unwrap();
        nodes.clear();
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        for item in iter {
            let (key, value) = item.unwrap();
            if key.starts_with(b"node_") {
                if let Ok(node) = bincode::deserialize::<Node>(&value) {
                    nodes.insert(node.dc_id.clone(), node);
                }
            }
        }
    }
}
