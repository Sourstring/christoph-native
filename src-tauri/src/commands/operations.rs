use tauri::Window;

use crate::state::ConnectionPool::CONNECTION_POOL;
use crate::sftp::{cancel_transfer, start_download, start_upload};

#[tauri::command]
pub async fn list_directory(
    connection_id: String,
    path: String,
) -> Result<Vec<crate::sftp::FileInfo>, String> {
    let client_arc = CONNECTION_POOL
        .get(&connection_id)
        .ok_or_else(|| "Connection not found".to_string())?;
    
    let client = client_arc.lock()
        .map_err(|e| format!("Failed to lock client: {}", e))?;
    
    client.list_directory(&path)
}

#[tauri::command]
pub async fn upload_file(
    connection_id: String,
    local_path: String,
    remote_path: String,
    window: Window,
) -> Result<String, String> {
    let client_arc = CONNECTION_POOL
        .get(&connection_id)
        .ok_or_else(|| "Connection not found".to_string())?;
    
    start_upload(local_path, remote_path, window, client_arc)
}

#[tauri::command]
pub async fn download_file(
    connection_id: String,
    remote_path: String,
    local_path: String,
    window: Window,
) -> Result<String, String> {
    let client_arc = CONNECTION_POOL
        .get(&connection_id)
        .ok_or_else(|| "Connection not found".to_string())?;
    
    start_download(remote_path, local_path, window, client_arc)
}

#[tauri::command]
pub async fn cancel_file_transfer(transfer_id: String) -> Result<(), String> {
    cancel_transfer(&transfer_id)
}

#[tauri::command]
pub async fn delete_file(
    connection_id: String,
    path: String,
    is_dir: bool,
) -> Result<(), String> {
    let client_arc = CONNECTION_POOL
        .get(&connection_id)
        .ok_or_else(|| "Connection not found".to_string())?;
    
    let client = client_arc.lock()
        .map_err(|e| format!("Failed to lock client: {}", e))?;
    
    client.delete(&path, is_dir)
}

#[tauri::command]
pub async fn create_directory(
    connection_id: String,
    path: String,
) -> Result<(), String> {
    let client_arc = CONNECTION_POOL
        .get(&connection_id)
        .ok_or_else(|| "Connection not found".to_string())?;
    
    let client = client_arc.lock()
        .map_err(|e| format!("Failed to lock client: {}", e))?;
    
    client.create_directory(&path)
}

#[tauri::command]
pub async fn rename_file(
    connection_id: String,
    old_path: String,
    new_path: String,
) -> Result<(), String> {
    let client_arc = CONNECTION_POOL
        .get(&connection_id)
        .ok_or_else(|| "Connection not found".to_string())?;
    
    let client = client_arc.lock()
        .map_err(|e| format!("Failed to lock client: {}", e))?;
    
    client.rename(&old_path, &new_path)
}