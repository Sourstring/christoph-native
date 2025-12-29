use tauri::State;
use uuid::Uuid;
use crate::{ConnectionPool, sftp::{ConnectionConfig, SftpClient}};

#[tauri::command]
pub async fn connect_sftp(
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    passphrase: Option<String>,
    private_key_path: Option<String>,
    pool: State<'_, ConnectionPool>,
) -> Result<String, String> {
    let config = ConnectionConfig { host: host.clone(), port, username, password, passphrase, private_key_path };
    
    let client = SftpClient::connect(config)
        .map_err(|e| e.to_string())?;
    
    let connection_id = Uuid::new_v4().to_string(); // or use UUID
    pool.add(connection_id.clone(), client);
    
    Ok(connection_id)
}

#[tauri::command]
pub async fn disconnect_sftp(
    connection_id: String,
    pool: State<'_, ConnectionPool>,
) -> Result<(), String> {
    pool.remove(&connection_id);
    Ok(())
}