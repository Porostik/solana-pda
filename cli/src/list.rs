use crate::storage::Storage;

pub fn list_command(storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
    let list = &storage.list;

    for wallet in list {
        println!("Wallet: {}, address: {}", wallet.name, wallet.pda);
    }

    Ok(())
}
