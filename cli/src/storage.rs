use std::{fs};

use borsh::io;
use serde::{Deserialize, Serialize};

use crate::constants::WALLETS_FILE_PATH;

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub name: String,
    pub pda: String,
    pub bump: u8
}

impl Wallet {
    pub fn new(name: String, pda: String, bump: u8) -> Wallet {
        Self { name, pda, bump }
    }
}

pub struct Storage {
    pub list: Vec<Wallet>
}

impl Storage {
    pub fn new() -> Result<Self, io::Error> {
        let list = Self::init_wallets()?;

        Ok(Self { list })
    }

    fn init_wallets() -> Result<Vec<Wallet>, io::Error> {
        let file_content = fs::read_to_string(WALLETS_FILE_PATH).unwrap_or("[]".to_string());
        let wallets = serde_json::from_str::<Vec<Wallet>>(&file_content)?;

        Ok(wallets)
    }

    pub fn save_new_wallet(&mut self, wallet: &Wallet) -> Result<(), io::Error> {
        if let Some(existed_wallet) = self.list.iter().find(|w| { w.name == wallet.name }) {
            Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Wallet {} already exist", existed_wallet.name)))
        } else {
            self.list.push(wallet.clone());
            let wallets_json = serde_json::to_string_pretty(&self.list)?;
            let _ = fs::write(WALLETS_FILE_PATH, wallets_json)?;
            Ok(())
        }
    }

    pub fn get_wallet(&self, name: &str) -> Option<Wallet> {
        if let Some(existed_wallet) = self.list.iter().find(|w| { w.name == name }) {
            Some(existed_wallet.clone())
        } else {
            None
        }
    }
}
