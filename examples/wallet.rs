use rust_client::{RpcType, Wallet};

fn main() {
    let wallet = Wallet::default(RpcType::Devnet);
    println!("{}", wallet);
    let block_height = wallet.client.get_block_height();
    println!("block_height: {:?}", block_height);

    let balance = wallet.balance(None);
    if balance < 5 * 1_000_000_000 {
        wallet.airdrop(None, 5);
    }
    
}