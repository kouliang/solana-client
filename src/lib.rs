pub mod wallet;
pub mod instruction_builder;
pub mod transaction_builder;
pub mod versioned_transaction_builder;

pub use wallet::RpcType;
pub use wallet::Wallet;

use solana_client::rpc_client::SerializableTransaction;

pub fn send_transaction(wallet: &Wallet, transaction: &impl SerializableTransaction) {
    let result = wallet.client.send_and_confirm_transaction(transaction);

    match result {
        Ok(sig) => loop {
            if let Ok(confirmed) = wallet.client.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(err) => println!("{}", err),
    }
}