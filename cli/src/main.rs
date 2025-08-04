use clap::{Parser, Subcommand};

use crate::{config::AppConfig, constants::RCP_URL, storage::Storage};

mod generate;
mod balance;
mod storage;
mod list;
mod send;
mod program_data;
mod config;
mod constants;

#[derive(Parser, Debug)]
#[command(name = "AppTool", version = "1.0", about = "A tool with subcommands")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate {
        name: String
    },

    Balance {
        name: String
    },

    List,

    Send {
        #[arg(short, long, value_delimiter = ':')]
        name: String,
        #[arg(short, long, value_delimiter = ':')]
        recipient: String,
        #[arg(short, long, value_delimiter = ':')]
        amount: f64
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let storage = &mut Storage::new()?;

    let app_config = &AppConfig::new(RCP_URL.to_string());

    match &cli.command {
        Commands::Generate { name } => generate::generate_command(name, storage, app_config).await,
        Commands::Balance { name } => balance::balance_command(name, storage, app_config).await,
        Commands::List => list::list_command(&storage),
        Commands::Send { name, recipient, amount } => send::send_command(name, recipient, *amount, storage, app_config).await,
    }
}
