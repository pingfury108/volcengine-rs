use clap::{Parser, Subcommand};
use log::error;
use std::env;
use std::process;

mod cli;

/// A CLI tool for interacting with Volcengine APIs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Commands for the Visual (CV) service
    Visual(cli::visual::VisualCommand),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Load .env file if it exists
    dotenv::dotenv().ok();

    // Helper closure to read env vars or exit
    let get_env_var = |key: &str| -> String {
        match env::var(key) {
            Ok(val) => val,
            Err(_) => {
                error!(
                    "Error: Environment variable '{}' not found. Please set it in your environment or a .env file.",
                    key
                );
                process::exit(1);
            }
        }
    };

    // Load credentials from environment variables
    let access_key = get_env_var("VOLCENGINE_ACCESS_KEY");
    let secret_key = get_env_var("VOLCENGINE_SECRET_KEY");

    let cli = Cli::parse();

    match cli.command {
        Commands::Visual(cmd) => cli::visual::handle(cmd, &access_key, &secret_key).await?,
    }

    Ok(())
}
