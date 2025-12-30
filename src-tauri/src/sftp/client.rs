// sftp_client.rs
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{net::TcpStream, path::Path};
use std::io::{Read, Write};

use once_cell::sync::Lazy;
use ssh2::{Session, Sftp};
use tauri::{Emitter, Window};
use uuid::Uuid;

use crate::sftp::utils::format_permissions;
use crate::sftp::{ConnectionConfig, FileInfo};

static TRANSFER_CANCEL_MAP: Lazy<Mutex<HashMap<String, Arc<AtomicBool>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct SftpClient {
    #[allow(dead_code)]
    session: Session,
    sftp: Sftp,
    #[allow(dead_code)]
    config: ConnectionConfig,
    connection_id: String,
}

impl SftpClient {

    pub fn connect(config: ConnectionConfig) -> Result<Self, String> {
        let connection_id = Uuid::new_v4().to_string();

        let tcp = TcpStream::connect(format!("{}:{}", config.host, config.port))
            .map_err(|e| format!("Failed to connect: {}", e))?;

        let mut session = Session::new()
            .map_err(|e| format!("Failed to create session: {}", e))?;
        session.set_tcp_stream(tcp);
        session
            .handshake()
            .map_err(|e| format!("SSH handshake failed: {}", e))?;

        if let Some(private_key_path) = &config.private_key_path {
            session
                .userauth_pubkey_file(
                    &config.username,
                    None,
                    Path::new(private_key_path),
                    config.passphrase.as_deref(),
                )
                .map_err(|e| format!("Key authentication failed: {}", e))?;
        } else if let Some(password) = &config.password {
            session
                .userauth_password(&config.username, password)
                .map_err(|e| format!("Password authentication failed: {}", e))?;
        } else {
            return Err("No authentication method provided".to_string());
        }

        if !session.authenticated() {
            return Err("Authentication failed".to_string());
        }

        let sftp = session
            .sftp()
            .map_err(|e| format!("Failed to create SFTP session: {}", e))?;

        Ok(Self {
            session,
            sftp,
            config,
            connection_id,
        })
    }

    pub fn connection_id(&self) -> &str {
        &self.connection_id
    }

    pub fn list_directory(&self, path: &str) -> Result<Vec<FileInfo>, String> {
        let dir_path = Path::new(path);
        let entries = self.sftp
            .readdir(dir_path)
            .map_err(|e| format!("Error reading directory: {}", e))?;

        let mut files = Vec::new();
        for (file_path, stat) in entries {
            let name = file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            
            if name.starts_with('.') && name != ".." {
                continue;
            }

            files.push(FileInfo {
                name,
                path: file_path.to_string_lossy().replace("\\", "/"),
                is_dir: stat.is_dir(),
                size: stat.size.unwrap_or(0),
                modified: stat.mtime.unwrap_or(0),
                permissions: format_permissions(stat.perm.unwrap_or(0)),
            });
        }

        files.sort_by(|a, b| {
            if a.name == ".." {
                return std::cmp::Ordering::Less;
            }
            if b.name == ".." {
                return std::cmp::Ordering::Greater;
            }
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        Ok(files)
    }

    fn upload_file_blocking(
        &self,
        local_path: &str,
        remote_path: &str,
        window: &Window,
        transfer_id: &str,
        cancel_flag: &Arc<AtomicBool>,
    ) -> Result<(), String> {
        let mut local_file = std::fs::File::open(local_path)
            .map_err(|e| format!("Failed to open local file: {}", e))?;

        let total_size = local_file
            .metadata()
            .map_err(|e| format!("Failed to stat local file: {}", e))?
            .len();

        let mut remote_file = self.sftp
            .create(Path::new(remote_path))
            .map_err(|e| format!("Failed to create remote file: {}", e))?;

        let mut buffer = [0u8; 8192];
        let mut transferred = 0u64;

        loop {
            if cancel_flag.load(Ordering::Relaxed) {
                window.emit(
                    "transfer_cancelled",
                    serde_json::json!({
                        "transfer_id": transfer_id,
                        "type": "upload"
                    }),
                ).ok();
                return Err("Transfer cancelled".to_string());
            }

            let n = local_file
                .read(&mut buffer)
                .map_err(|e| format!("Read error: {}", e))?;

            if n == 0 {
                break;
            }

            remote_file
                .write_all(&buffer[..n])
                .map_err(|e| format!("Write error: {}", e))?;

            transferred += n as u64;

            window.emit(
                "upload_progress",
                serde_json::json!({
                    "connection_id": self.connection_id,
                    "path": remote_path,
                    "transferred": transferred,
                    "total": total_size,
                    "type": "upload",
                    "transfer_id": transfer_id
                }),
            ).ok();
        }

        window.emit(
            "process_finished",
            serde_json::json!({
                "connection_id": self.connection_id,
                "path": remote_path,
                "type": "upload",
                "transfer_id": transfer_id
            }),
        ).ok();

        Ok(())
    }

    fn download_file_blocking(
        &self,
        remote_path: &str,
        local_path: &str,
        window: &Window,
        transfer_id: &str,
        cancel_flag: &Arc<AtomicBool>,
    ) -> Result<(), String> {
        let mut remote_file = self.sftp
            .open(Path::new(remote_path))
            .map_err(|e| format!("Failed to open remote file: {}", e))?;

        let total_size = remote_file
            .stat()
            .map_err(|e| format!("Failed to stat remote file: {}", e))?
            .size
            .unwrap_or(0);

        let mut local_file = std::fs::File::create(local_path)
            .map_err(|e| format!("Failed to create local file: {}", e))?;

        let mut buffer = [0u8; 8192];
        let mut transferred = 0u64;

        loop {
            if cancel_flag.load(Ordering::Relaxed) {
                drop(local_file);
                std::fs::remove_file(local_path).ok();
                
                window.emit(
                    "transfer_cancelled",
                    serde_json::json!({
                        "transfer_id": transfer_id,
                        "type": "download"
                    }),
                ).ok();
                return Err("Transfer cancelled".to_string());
            }

            let n = remote_file
                .read(&mut buffer)
                .map_err(|e| format!("Read error: {}", e))?;

            if n == 0 {
                break;
            }

            local_file
                .write_all(&buffer[..n])
                .map_err(|e| format!("Write error: {}", e))?;

            transferred += n as u64;

            window.emit(
                "download_progress",
                serde_json::json!({
                    "connection_id": self.connection_id,
                    "path": remote_path,
                    "transferred": transferred,
                    "total": total_size,
                    "type": "download",
                    "transfer_id": transfer_id
                }),
            ).ok();
        }

        local_file
            .sync_all()
            .map_err(|e| format!("Failed to sync file: {}", e))?;

        window.emit(
            "process_finished",
            serde_json::json!({
                "connection_id": self.connection_id,
                "path": remote_path,
                "type": "download",
                "transfer_id": transfer_id
            }),
        ).ok();

        Ok(())
    }

    pub fn delete(&self, path: &str, is_dir: bool) -> Result<(), String> {
        let path = Path::new(path);
        
        if is_dir {
            self.sftp.rmdir(path)
                .map_err(|e| format!("Failed to delete directory: {}", e))?;
        } else {
            self.sftp.unlink(path)
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }
        
        Ok(())
    }

    pub fn create_directory(&self, path: &str) -> Result<(), String> {
        self.sftp.mkdir(Path::new(path), 0o755)
            .map_err(|e| format!("Failed to create directory: {}", e))
    }

    pub fn rename(&self, old_path: &str, new_path: &str) -> Result<(), String> {
        self.sftp.rename(Path::new(old_path), Path::new(new_path), None)
            .map_err(|e| format!("Failed to rename: {}", e))
    }

}


pub fn cancel_transfer(transfer_id: &str) -> Result<(), String> {
    let map = TRANSFER_CANCEL_MAP.lock().unwrap();
    if let Some(cancel_flag) = map.get(transfer_id) {
        cancel_flag.store(true, Ordering::Relaxed);
        Ok(())
    } else {
        Err("Transfer not found".to_string())
    }
}

pub fn start_upload(
    local_path: String,
    remote_path: String,
    window: Window,
    client_arc: Arc<Mutex<SftpClient>>,
) -> Result<String, String> {
    let transfer_id = Uuid::new_v4().to_string();
    let cancel_flag = Arc::new(AtomicBool::new(false));

    TRANSFER_CANCEL_MAP
        .lock()
        .unwrap()
        .insert(transfer_id.clone(), cancel_flag.clone());

    let transfer_id_clone = transfer_id.clone();

    tokio::task::spawn_blocking(move || {
        let result = {
            let client = client_arc.lock()
                .map_err(|e| format!("Failed to lock client: {}", e));
            
            match client {
                Ok(client) => {
                    client.upload_file_blocking(
                        &local_path,
                        &remote_path,
                        &window,
                        &transfer_id_clone,
                        &cancel_flag,
                    )
                }
                Err(e) => Err(e),
            }
        };

        TRANSFER_CANCEL_MAP.lock().unwrap().remove(&transfer_id_clone);

        if let Err(e) = result {
            window.emit(
                "transfer_error",
                serde_json::json!({
                    "transfer_id": transfer_id_clone,
                    "error": e,
                    "type": "upload"
                }),
            ).ok();
        }
    });

    Ok(transfer_id)
}

pub fn start_download(
    remote_path: String,
    local_path: String,
    window: Window,
    client_arc: Arc<Mutex<SftpClient>>,
) -> Result<String, String> {
    let transfer_id = Uuid::new_v4().to_string();
    let cancel_flag = Arc::new(AtomicBool::new(false));

    TRANSFER_CANCEL_MAP
        .lock()
        .unwrap()
        .insert(transfer_id.clone(), cancel_flag.clone());

    let transfer_id_clone = transfer_id.clone();
    let local_path_clone = local_path.clone();

    tokio::task::spawn_blocking(move || {
        let result = {
            let client = client_arc.lock()
                .map_err(|e| format!("Failed to lock client: {}", e));
            
            match client {
                Ok(client) => {
                    client.download_file_blocking(
                        &remote_path,
                        &local_path,
                        &window,
                        &transfer_id_clone,
                        &cancel_flag,
                    )
                }
                Err(e) => Err(e),
            }
        };

        TRANSFER_CANCEL_MAP.lock().unwrap().remove(&transfer_id_clone);

        if let Err(e) = result {
            std::fs::remove_file(&local_path_clone).ok();
            
            window.emit(
                "transfer_error",
                serde_json::json!({
                    "transfer_id": transfer_id_clone,
                    "error": e,
                    "type": "download"
                }),
            ).ok();
        }
    });

    Ok(transfer_id)
}