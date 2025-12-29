// connection_pool.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

use crate::sftp::SftpClient;

pub struct ConnectionPool {
    connections: Mutex<HashMap<String, Arc<Mutex<SftpClient>>>>,
}

impl ConnectionPool {
    pub fn new() -> Self {
        Self {
            connections: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(&self, id: String, client: SftpClient) -> Arc<Mutex<SftpClient>> {
        let client = Arc::new(Mutex::new(client));
        self.connections.lock().unwrap().insert(id.clone(), client.clone());
        client
    }

    pub fn get(&self, id: &str) -> Option<Arc<Mutex<SftpClient>>> {
        self.connections.lock().unwrap().get(id).cloned()
    }

    pub fn remove(&self, id: &str) -> Option<Arc<Mutex<SftpClient>>> {
        self.connections.lock().unwrap().remove(id)
    }

    pub fn list_connections(&self) -> Vec<String> {
        self.connections.lock().unwrap().keys().cloned().collect()
    }
}

pub static CONNECTION_POOL: Lazy<ConnectionPool> = Lazy::new(|| ConnectionPool::new());