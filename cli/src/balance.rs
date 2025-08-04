use std::str::FromStr;

use crate::{config::AppConfig, storage::Storage};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

pub async fn balance_command(name: &str, storage: &Storage, app_config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(wallet) = storage.get_wallet(name) {
        let connection = &app_config.rcp_client;

        let wallet_pda = &wallet.pda;
        let wallet_pubkey = Pubkey::from_str(wallet_pda)?;

        let account = connection.get_account(&wallet_pubkey).await?;
        let balance = connection.get_balance(&wallet_pubkey).await?;

        let data_size = account.data.len();

        let min_rent = connection.get_minimum_balance_for_rent_exemption(data_size).await?;

        let sol_balance = (balance as f64 - min_rent as f64) / LAMPORTS_PER_SOL as f64;

        println!("Balance for wallet: {} - {}", wallet.name, sol_balance)

    } else {
       println!("Wallet not found")
    }

    Ok(())
}
