use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::{system_program};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use crate::config::AppConfig;
use crate::storage::{self, Storage}; 
use crate::constants::{KEYPAIR_FILE_PATH, PROGRAM_ADDRESS}; 
use crate::program_data::{get_program_data, ProgramCommand};

pub async fn generate_command(name: &str, storage: &mut Storage, app_config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(_existed) = storage.get_wallet(name) {
        println!("Wallet already exist");
        return Ok(())
    }

    let connection = &app_config.rcp_client;

    let program_address = Pubkey::from_str_const(PROGRAM_ADDRESS);
    let keypair = read_keypair_file(KEYPAIR_FILE_PATH)?;
    let user_pubkey = keypair.pubkey();
    let seeds = &[b"user", name.as_bytes(), user_pubkey.as_ref()];
    let (pda, bump) = Pubkey::find_program_address(seeds, &program_address);

    let data = get_program_data(ProgramCommand::Initialize { name: name.to_string() });

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let instruction = Instruction {
        program_id: program_address,
        data,
        accounts: vec![
            AccountMeta::new(keypair.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ]
    };

    let mut tx = Transaction::new_with_payer(&[instruction], Some(&keypair.pubkey()));

    tx.sign(&[&keypair], recent_blockhash);
    
    let _signature = connection.send_and_confirm_transaction(&tx).await?;

    if let Ok(_) = connection.get_account_data(&pda).await {
        let wallet = storage::Wallet::new(name.to_string(), pda.to_string(), bump);
        let save_wallet_result = storage.save_new_wallet(&wallet);

        match save_wallet_result {
            Ok(_) => println!("Wallet {} success created", wallet.name),
            Err(err) => println!("Error: {}", err)
        }
    }

    Ok(())
}
