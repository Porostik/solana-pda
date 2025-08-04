use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

pub struct AppConfig {
    pub rcp_client: RpcClient,
}

impl AppConfig {
    pub fn new(rcp_url: String) -> Self {
        Self { rcp_client: create_rcp_client(rcp_url) }
    }
}

fn create_rcp_client(url: String) -> RpcClient {
    RpcClient::new_with_commitment(url, CommitmentConfig::confirmed())
}
