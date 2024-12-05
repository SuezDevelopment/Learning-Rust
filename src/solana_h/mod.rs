use solana_sdk::{signature::Keypair, system_instruction, message::Message};
use solana_program::instruction::Instruction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use tokio::sync::RwLock;

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
const TX_FEE: u64 = 5000;
const RENT_EXEMPT_MIN: u64 = 53;

#[derive(Debug, Clone)]
pub struct WalletState {
    pub connected_wallet: Option<Pubkey>,
    pub balance: u64,
}

pub struct SolanaClient {
    rpc_client: RpcClient,
    receiver_keypair: Keypair,
    sender_publicKey: Pubkey,
    wallet_state: Arc<RwLock<WalletState>>,
}

impl SolanaClient {
    pub fn new(rpc_url: &str, receiver_keypair: Keypair, sender_publicKey: Pubkey) -> Self {
        let rpc_client = RpcClient::new(rpc_url.to_string());
        let wallet_state = Arc::new(
            RwLock::new(WalletState {
                connected_wallet: None,
                balance: 0,
            })
        );

        SolanaClient {
            rpc_client,
            receiver_keypair,
            sender_publicKey,
            wallet_state,
        }
    }

    pub async fn handle_wallet_connection(&self, wallet_pubkey: Pubkey) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Processing wallet connection: {}", wallet_pubkey);
        let balance = self.rpc_client.get_balance(&wallet_pubkey)?;
        
        self.update_wallet_state(wallet_pubkey, balance).await?;
        
        log::info!("Wallet connected successfully with balance: {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);
        Ok(())
    }

    async fn update_wallet_state(&self, pubkey: Pubkey, balance: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = self.wallet_state.write().await;
        state.connected_wallet = Some(pubkey);
        state.balance = balance;
        Ok(())
    }

    pub async fn get_connected_wallet(&self) -> Option<Pubkey> {
        self.wallet_state.read().await.connected_wallet
    }

    pub fn check_balance(&self) -> u64 {
        self.rpc_client.get_balance(&self.receiver_keypair.pubkey()).unwrap_or(0)
    }

}
