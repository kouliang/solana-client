use rust_client::{RpcType, Wallet, keypair};
use rust_client::instruction_builder;
use rust_client::transaction_builder;

use solana_sdk::signer::Signer;

fn main() {
    let wallet = Wallet::default(RpcType::Devnet);

    let new = keypair::new();
    println!("new address: {:?}", new.pubkey().to_string());
    let instruction1 = instruction_builder::create_account(&wallet, &new.pubkey(), &solana_program::system_program::id(), 10);

    let signing_keypairs = &[&wallet.payer, &new];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    wallet.send_transaction(&transaction);
}