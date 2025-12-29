mod connection;
mod operations;

pub use connection::{connect_sftp, disconnect_sftp};
pub use operations::{list_files, download_file, upload_file};