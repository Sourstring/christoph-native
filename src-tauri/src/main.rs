mod commands;
mod sftp;
mod state;

use state::ConnectionPool;

fn main() {
    tauri::Builder::default()
        .manage(ConnectionPool::new()) // Shared state
        .invoke_handler(tauri::generate_handler![
            commands::connection::connect_sftp,
            commands::connection::disconnect_sftp,
            commands::file_ops::list_files,
            commands::file_ops::download_file,
            commands::file_ops::upload_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}