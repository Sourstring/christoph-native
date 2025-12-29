use tauri::State;

use crate::{ConnectionPool, sftp::{ FileInfo }};

#[tauri::command]
pub async fn list_files(
    connection_id: String,
    path: String,
    pool: State<'_, ConnectionPool>,
) -> Result<Vec<FileInfo>, String> {
    let client = pool.get(&connection_id)
        .ok_or("Connection not found")?;
    
    client.list_directory(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_file(
    connection_id: String,
    remote_path: String,
    local_path: String,
    pool: State<'_, ConnectionPool>,
) -> Result<(), String> {
    let client = pool.get(&connection_id)
        .ok_or("Connection not found")?;
    
    client.download_file(&remote_path, &local_path)
        .map_err(|e| e.to_string())
}


#[tauri::command]
pub async fn upload_file(connection_id: String,
    remote_path: String,
    local_path: String,
    pool: State<'_, ConnectionPool>,) -> Result<(), String> {
    let client = pool.get(&connection_id)
        .ok_or("Connection not found")?;
    
    client.upload_file(&remote_path, &local_path)
        .map_err(|e| e.to_string())
}