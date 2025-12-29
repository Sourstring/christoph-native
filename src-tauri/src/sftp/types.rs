use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub private_key_path: Option<String>,
    pub passphrase: Option<String>,
    pub password: Option<String>
}

#[derive(Serialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: u64,
    pub permissions: String
}

#[derive(Serialize, Clone)]
pub struct DirectoryListing {
    pub path: String,
    pub files: Vec<FileInfo>,
}