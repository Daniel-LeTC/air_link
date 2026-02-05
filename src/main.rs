use air_link::cli::{Cli, actions};
use air_link::Result;
use clap::Parser;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // env::set_var is unsafe in newer Rust versions due to thread safety concerns.
    // Since we are at the very beginning of the program, it's safe to call.
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
        
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "info");
        }
    }

    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Air-Link is starting...");

    // Parse CLI arguments
    let cli = Cli::parse();

    // Handle the command
    if let Err(e) = actions::handle_command(cli) {
        eprintln!("🔥 CRITICAL ERROR: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
