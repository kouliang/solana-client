use rust_client::{RpcType, Wallet, wallet};
use rust_client::instruction_builder;
use rust_client::transaction_builder;

fn main() {
    let wallet = Wallet::default(RpcType::Devnet);

    let to = wallet::pubkey_from_str("BdzfzYqiSL5sAarVnUxnYwZrruASq5gziizVVzFncVfp");
    let instruction1 = instruction_builder::transfer_to(&wallet, &to, 1000000000);

    let signing_keypairs = &[&wallet.payer];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    rust_client::send_transaction(&wallet, &transaction);
}