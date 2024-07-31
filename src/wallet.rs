use std::str::FromStr;
use std::fmt::Display;
use std::fmt;

use solana_client::rpc_client::RpcClient;

use solana_sdk::signature::Keypair;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::keypair;
use solana_sdk::signer::Signer;
use solana_sdk::native_token::{lamports_to_sol, LAMPORTS_PER_SOL};

use solana_program::pubkey::Pubkey;

pub struct Wallet {
    pub client: RpcClient,
    pub payer: Keypair
}

pub enum RpcType {
    Localhost,
    Testnet,
    Devnet,
    Mainnet,
    Custom(String)
}

const DEFAULT_KEYPAIR_PATH: &str = "/Users/kou/.config/solana/id.json";

impl Wallet {
    pub fn new(keypair_path: &std::path::Path, rpc_type: RpcType) -> Self {
        let url = match rpc_type {
            RpcType::Localhost => "http://localhost:8899".to_string(),
            RpcType::Testnet => "https://api.testnet.solana.com".to_string(),
            RpcType::Devnet => "https://api.devnet.solana.com".to_string(),
            RpcType::Mainnet => "https://api.mainnet-beta.solana.com".to_string(),
            RpcType::Custom(url) => url
        };

        Wallet {
            client: RpcClient::new_with_commitment(url, CommitmentConfig::confirmed()),
            payer: keypair_frome_file(keypair_path)
        }
    }

    pub fn default(rpc_type: RpcType) -> Self {
        let path = std::path::Path::new(DEFAULT_KEYPAIR_PATH);
        Self::new(path, rpc_type)
    }

    pub fn pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    pub fn get_rent(&self, data_len: usize) -> u64 {
        self.client.get_minimum_balance_for_rent_exemption(data_len).unwrap()
    }

    pub fn airdrop(&self, pubkey: Option<&Pubkey>, amount: u64) {
        let selfpubkey = self.pubkey();
        let pubkey = pubkey.unwrap_or(&selfpubkey);
        match self.client.request_airdrop(pubkey, amount*LAMPORTS_PER_SOL) {
            Ok(sig) => loop {
                if let Ok(confirmed) = self.client.confirm_transaction(&sig) {
                    if confirmed {
                        println!("Airdrop Status: {} sig: {}", confirmed, sig);
                        break;
                    }
                }
            },
            Err(_) => println!("Airdrop Error"),
        };
    }

    pub fn balance(&self, pubkey: Option<&Pubkey>) -> u64 {
        let selfpubkey = self.pubkey();
        let pubkey = pubkey.unwrap_or(&selfpubkey);
        let balance = self.client.get_balance(pubkey).unwrap(); // 查询余额
        println!("balance: {:?} SOL", lamports_to_sol(balance));
        balance
    }
}

impl Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address:{}\nrpc:{}", self.payer.pubkey(), self.client.url())
    }
}

pub fn keypair_new() -> Keypair {
    Keypair::new()
}

pub fn keypair_from_bytes(bytes: &[u8]) -> Keypair {
    Keypair::from_bytes(bytes).unwrap()
}

pub fn keypair_from_base58_string(s: &str) -> Keypair {
    Keypair::from_base58_string(s)
}

pub fn keypair_frome_file(path: &std::path::Path) -> Keypair {
    keypair::read_keypair_file(path).unwrap()
}

pub fn pubkey_from_str(s: &str) -> Pubkey {
    Pubkey::from_str(s).unwrap()
}
