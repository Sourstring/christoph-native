use std::io::{self, Write};
use christoph_lib::sftp::{SftpClient, ConnectionConfig};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║        SFTP Test CLI v1.0              ║");
    println!("╚════════════════════════════════════════╝\n");

    // Get connection details
    let config = get_connection_config();

    // Connect
    println!("\n⏳ Connecting to {}:{}...", config.host, config.port);
    let client = match SftpClient::connect(config) {
        Ok(c) => {
            println!("✓ Connected successfully!");
            println!("  Connection ID: {}\n", c.connection_id());
            c
        }
        Err(e) => {
            eprintln!("✗ Connection failed: {}", e);
            eprintln!("\n💡 Tips:");
            eprintln!("  - Check if the server is running");
            eprintln!("  - Verify the port number (Docker SFTP usually uses 2222)");
            eprintln!("  - Check your credentials");
            return;
        }
    };

    // Start interactive shell
    interactive_shell(client);
}

fn get_connection_config() -> ConnectionConfig {
    println!("Enter connection details:\n");

    let host = prompt("Host", Some("localhost"));
    let port = prompt_number("Port", 2222);
    let username = prompt("Username", Some("foo"));
    
    println!("\nAuthentication method:");
    println!("  1. Password");
    println!("  2. Private Key");
    let auth_method = prompt("Choose (1 or 2)", Some("1"));

    let (password, private_key_path, passphrase) = if auth_method == "2" {
        let key_path = prompt("Private key path", None);
        let passphrase = prompt_optional("Passphrase (optional)");
        (None, Some(key_path), passphrase)
    } else {
        let password = prompt("Password", Some("pass"));
        (Some(password), None, None)
    };

    ConnectionConfig {
        host,
        port,
        username,
        password,
        private_key_path,
        passphrase,
    }
}

fn interactive_shell(client: SftpClient) {
    let mut current_path = String::from("/home/foo");
    
    println!("Type 'help' for available commands\n");

    loop {
        print!("\x1b[1;36m{}>\x1b[0m ", current_path);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            "ls" | "dir" => cmd_ls(&client, args, &current_path),
            "cd" => cmd_cd(&client, args, &mut current_path),
            "pwd" => cmd_pwd(&current_path),
            "mkdir" => cmd_mkdir(&client, args, &current_path),
            "rm" | "del" => cmd_rm(&client, args, &current_path),
            "rmdir" => cmd_rmdir(&client, args, &current_path),
            "rename" | "mv" => cmd_rename(&client, args, &current_path),
            "help" | "?" => cmd_help(),
            "clear" | "cls" => print!("\x1b[2J\x1b[H"),
            "exit" | "quit" | "q" => {
                println!("\n👋 Goodbye!");
                break;
            }
            _ => println!("❌ Unknown command '{}'. Type 'help' for available commands.", command),
        }
    }
}

fn cmd_ls(client: &SftpClient, args: &[&str], current_path: &str) {
    let path = if args.is_empty() {
        current_path
    } else {
        args[0]
    };

    let display_path = resolve_path(path, current_path);

    println!("  🔍 Trying to list: {}", display_path);  // Debug line

    match client.list_directory(&display_path) {
        Ok(files) => {
            if files.is_empty() {
                println!("  (empty directory)");
                return;
            }

            println!("\n  📁 Listing: {}\n", display_path);
            println!("  {:<12} {:<15} {:<10} Name", "Permissions", "Size", "Modified");
            println!("  {}", "─".repeat(70));

            for file in files {
                let icon = if file.is_dir { "📁" } else { "📄" };
                let size = if file.is_dir {
                    "DIR".to_string()
                } else {
                    format_size(file.size)
                };
                let modified = format_timestamp(file.modified);
                
                println!("  {:<12} {:<15} {:<10} {} {}",
                    file.permissions,
                    size,
                    modified,
                    icon,
                    file.name
                );
            }
            println!();
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            println!("\n💡 Try these commands to explore:");
            println!("  ls /");
            println!("  ls .");
            println!("  ls ..");
            println!("  ls /home");
            println!("  ls /upload");
        }
    }
}

fn cmd_cd(client: &SftpClient, args: &[&str], current_path: &mut String) {
    if args.is_empty() {
        println!("Usage: cd <directory>");
        return;
    }

    let new_path = resolve_path(args[0], current_path);

    // Verify the directory exists
    match client.list_directory(&new_path) {
        Ok(_) => {
            *current_path = new_path.clone();
            println!("✓ Changed directory to: {}", new_path);
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn cmd_pwd(current_path: &str) {
    println!("{}", current_path);
}

fn cmd_mkdir(client: &SftpClient, args: &[&str], current_path: &str) {
    if args.is_empty() {
        println!("Usage: mkdir <directory_name>");
        return;
    }

    let dir_path = resolve_path(args[0], current_path);

    match client.create_directory(&dir_path) {
        Ok(_) => println!("✓ Directory created: {}", dir_path),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn cmd_rm(client: &SftpClient, args: &[&str], current_path: &str) {
    if args.is_empty() {
        println!("Usage: rm <file_name>");
        return;
    }

    let file_path = resolve_path(args[0], current_path);

    match client.delete(&file_path, false) {
        Ok(_) => println!("✓ File deleted: {}", file_path),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn cmd_rmdir(client: &SftpClient, args: &[&str], current_path: &str) {
    if args.is_empty() {
        println!("Usage: rmdir <directory_name>");
        return;
    }

    let dir_path = resolve_path(args[0], current_path);

    match client.delete(&dir_path, true) {
        Ok(_) => println!("✓ Directory deleted: {}", dir_path),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn cmd_rename(client: &SftpClient, args: &[&str], current_path: &str) {
    if args.len() < 2 {
        println!("Usage: rename <old_name> <new_name>");
        return;
    }

    let old_path = resolve_path(args[0], current_path);
    let new_path = resolve_path(args[1], current_path);

    match client.rename(&old_path, &new_path) {
        Ok(_) => println!("✓ Renamed: {} → {}", old_path, new_path),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn cmd_help() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    Available Commands                      ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    let commands = vec![
        ("ls, dir [path]", "List directory contents"),
        ("cd <path>", "Change current directory"),
        ("pwd", "Print working directory"),
        ("mkdir <name>", "Create a new directory"),
        ("rm <file>", "Delete a file"),
        ("rmdir <dir>", "Delete a directory"),
        ("rename <old> <new>", "Rename file or directory"),
        ("clear, cls", "Clear the screen"),
        ("help, ?", "Show this help message"),
        ("exit, quit, q", "Exit the program"),
    ];

    for (cmd, desc) in commands {
        println!("  {:<20} - {}", cmd, desc);
    }
    
    println!("\n💡 Tips:");
    println!("  • Use absolute paths starting with /");
    println!("  • Use relative paths from current directory");
    println!("  • Use .. to go up one directory");
    println!("  • Use . for current directory\n");
}

// Helper functions

fn resolve_path(path: &str, current_path: &str) -> String {
    if path.starts_with('/') {
        // Absolute path
        path.to_string()
    } else if path == "." {
        // Current directory
        current_path.to_string()
    } else if path == ".." {
        // Parent directory
        let parts: Vec<&str> = current_path.rsplitn(2, '/').collect();
        if parts.len() > 1 && !parts[1].is_empty() {
            parts[1].to_string()
        } else {
            "/".to_string()
        }
    } else {
        // Relative path
        format!("{}/{}", current_path.trim_end_matches('/'), path)
    }
}

fn prompt(label: &str, default: Option<&str>) -> String {
    if let Some(def) = default {
        print!("  {} [{}]: ", label, def);
    } else {
        print!("  {}: ", label);
    }
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() {
        default.unwrap_or("").to_string()
    } else {
        input.to_string()
    }
}

fn prompt_optional(label: &str) -> Option<String> {
    print!("  {} (press Enter to skip): ", label);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() {
        None
    } else {
        Some(input.to_string())
    }
}

fn prompt_number(label: &str, default: u16) -> u16 {
    print!("  {} [{}]: ", label, default);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() {
        default
    } else {
        input.parse().unwrap_or(default)
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    
    let datetime = UNIX_EPOCH + Duration::from_secs(timestamp);
    let now = SystemTime::now();
    
    if let Ok(duration) = now.duration_since(datetime) {
        let secs = duration.as_secs();
        if secs < 60 {
            format!("{}s ago", secs)
        } else if secs < 3600 {
            format!("{}m ago", secs / 60)
        } else if secs < 86400 {
            format!("{}h ago", secs / 3600)
        } else {
            format!("{}d ago", secs / 86400)
        }
    } else {
        "future".to_string()
    }
}