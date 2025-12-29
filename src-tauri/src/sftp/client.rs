use std::sync::Arc;
use std::{fmt::format, net::TcpStream, path::Path, sync::Mutex};

use ssh2::{Session, Sftp};
use uuid::Uuid;
use crate::sftp::utils::format_permissions;
use crate::sftp::{ConnectionConfig, FileInfo};
use crate::ConnectionPool::CONNECTION_POOL;

pub struct SftpClient{
    session: Session,
    sftp: Sftp,
    config: ConnectionConfig,
    connection_id: String
}

impl SftpClient{
    
    pub fn connect(config: ConnectionConfig) -> Result<Arc<Mutex<Self>>, String> {
        let connection_id = Uuid::new_v4().to_string();

        let tcp = TcpStream::connect(format!("{}:{}", config.host, config.port))
            .map_err(|e| format!("Failed to connect: {}", e))?;

        // 2. Create SSH session
        let mut session = Session::new().map_err(|e| format!("Failed to create session: {}", e))?;
        session.set_tcp_stream(tcp);
        session.handshake().map_err(|e| format!("SSH handshake failed: {}", e))?;

        // 3. Authenticate
        if let Some(private_key_path) = &config.private_key_path{
            session
                .userauth_pubkey_file(&config.username, None, Path::new(private_key_path), config.passphrase.as_deref())
                .map_err(|e| format!("key authentication failed: {}", e))?;
        }else if let Some(password) = &config.password{
            session
                .userauth_password(&config.username, password)
                .map_err(|e| format!("Password authentication failed: {}", e))?;
        }else{
            return Err("No authentication method provided".to_string());
        }

        if !session.authenticated(){
            return Err("Authentication failed".to_string());
        }

        let sftp = session.sftp()
            .map_err(|e| format!("Failed to create SFTP session: {}", e))?;

        let client = Self {
            session,
            sftp,
            config,
            connection_id: connection_id.clone(),
        };

        Ok(CONNECTION_POOL.add(connection_id, client))

    }

    pub fn list_directory(&self, path: &str) -> Result<Vec<FileInfo>, String> {
        let sftp = &self.sftp;

        let mut files = Vec::new();
        let dir_path = Path::new(&path);
        let entries = sftp.readdir(dir_path).map_err(|e| format!("Error reading directory: {}", e))?;

        for(file_path, stat) in entries{
            let name = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if name.starts_with('.') && name != ".."{
                continue
            }

            files.push( FileInfo {
                name,
                path: file_path.to_string_lossy().replace("\\", "/"),
                is_dir: stat.is_dir(),
                size: stat.size.unwrap_or(0),
                modified: stat.mtime.unwrap_or(0),
                permissions: format_permissions(stat.perm.unwrap_or(0))
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

    pub fn download_file(&self, remote_path: &str, local_path: &str) -> Result<(), String> {
        // Download logic
    }

    pub fn upload_file(&self, local_path: &str, remote_path: &str) -> Result<(), String> {
        // Upload logic
    }

    pub fn delete(&self, path: &str) -> Result<(), String> {
        // Delete file or directory
    }

}