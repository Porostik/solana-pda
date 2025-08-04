use borsh::BorshSerialize;
use sha2::Digest;

pub enum ProgramCommand {
    Initialize { name: String },
    Transfer { amount: f64, }
}

pub fn get_program_data(command: ProgramCommand) -> Vec<u8> {
    let (hash, args) = match command {
        ProgramCommand::Initialize { name } =>  {
            let hash = sha2::Sha256::digest(b"global:initialize");
            let args = get_initialize_args(name);

            (hash, args)
        },
        ProgramCommand::Transfer { amount } =>  {
            let hash = sha2::Sha256::digest(b"global:transfer");
            let args = get_transfer_args(amount);

            (hash, args)
        },
    };

    let discriminator = &hash[..8];

    let mut data = Vec::new();
    
    data.extend_from_slice(discriminator);
    data.extend_from_slice(&args);

    data

}

#[derive(BorshSerialize)]
struct InitializeArgs {
    name: String
}

fn get_initialize_args(name: String) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let args = InitializeArgs { name };
    let _ = args.serialize(&mut data);
    data
}


#[derive(BorshSerialize)]
struct TransferArgs {
    amount: f64,
}

fn get_transfer_args(amount: f64) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let args = TransferArgs { amount };
    let _ = args.serialize(&mut data);
    data
}
