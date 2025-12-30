use crate::state::connection_pool::ConnectionPool;

mod commands;
mod sftp;
mod state;

fn main() {
    tauri::Builder::default()
        .manage(ConnectionPool::new()) // Shared state
        .invoke_handler(tauri::generate_handler![
            // Connection commands
            commands::connection::connect_sftp,
            commands::connection::disconnect_sftp,
            // File operation commands
            commands::operations::list_directory,
            commands::operations::upload_file,
            commands::operations::download_file,
            commands::operations::cancel_file_transfer,
            commands::operations::delete_file,
            commands::operations::create_directory, 
            commands::operations::rename_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}