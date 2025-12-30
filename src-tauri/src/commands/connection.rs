use crate::{sftp::{ConnectionConfig, SftpClient}, state::connection_pool::{CONNECTION_POOL}};


#[tauri::command]
pub async fn connect_sftp(
    host: String,
    port: u16,
    username: String,
    password: Option<String>,
    passphrase: Option<String>,
    private_key_path: Option<String>,
) -> Result<String, String> {
    let config = ConnectionConfig {
        host,
        port,
        username,
        password,
        passphrase,
        private_key_path,
    };
    
    let client = SftpClient::connect(config)?;
    let connection_id = client.connection_id().to_string();
    CONNECTION_POOL.add(connection_id.clone(), client);
    
    Ok(connection_id)
}


#[tauri::command]
pub async fn disconnect_sftp(connection_id: String) -> Result<(), String> {
    CONNECTION_POOL
        .remove(&connection_id)
        .ok_or_else(|| format!("Connection {} not found", connection_id))?;
    Ok(())
}