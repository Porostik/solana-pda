use std::str::FromStr;

use solana_sdk::{instruction::{AccountMeta, Instruction}, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::read_keypair_file, signer::Signer, system_program, transaction::Transaction};

use crate::{config::AppConfig, constants::{KEYPAIR_FILE_PATH, PROGRAM_ADDRESS}, program_data::get_program_data, storage::Storage};

pub async fn send_command(name: &str, recipient: &str, amount: f64, storage: &Storage, app_config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(wallet) = storage.get_wallet(name) {

        let client = &app_config.rcp_client;
        
        let program_address = Pubkey::from_str_const(PROGRAM_ADDRESS);
        let user_pubkey = Pubkey::from_str(&wallet.pda)?;
        let keypair = read_keypair_file(KEYPAIR_FILE_PATH)?;
        let recipient_pubkey = Pubkey::from_str(recipient)?;
        let data = get_program_data(
            crate::program_data::ProgramCommand::Transfer { 
                amount: amount * LAMPORTS_PER_SOL as f64,
            }
        );

        let instructions = Instruction {
            program_id: program_address,
            data,
            accounts: vec![
                AccountMeta::new(keypair.pubkey(), true),
                AccountMeta::new(user_pubkey, false),
                AccountMeta::new(recipient_pubkey, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ]
        };

        let blockhash = client.get_latest_blockhash().await?;

        let mut tx = Transaction::new_with_payer(&[instructions], Some(&keypair.pubkey()));

        tx.sign(&[&keypair], blockhash);

        let signature = client.send_and_confirm_transaction(&tx).await?;

        println!("{}", signature)
    } else {
        println!("Sender wallet not found")
    }


    Ok(())
}
