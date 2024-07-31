use crate::wallet::Wallet;

use solana_sdk::transaction::Transaction;
use solana_sdk::signature::Keypair;

use solana_program::message::legacy::Message;
use solana_program::instruction::Instruction;

pub fn signed_independent(wallet: &Wallet, instructions: &[Instruction], signing_keypairs:&[&Keypair]) -> Transaction {
    // message
    let latest_blockhash = wallet.client.get_latest_blockhash().unwrap();
    let message = Message::new_with_blockhash(
        instructions,
        Some(&wallet.pubkey()),
        &latest_blockhash
    );

    // transaction
    let mut transaction = Transaction::new_unsigned(message);

    //sign
    transaction.sign(signing_keypairs, transaction.message.recent_blockhash);

    transaction
}

pub fn signed_with_payer(wallet: &Wallet, instructions: &[Instruction], signing_keypairs:&[&Keypair]) -> Transaction {
    let latest_blockhash = wallet.client.get_latest_blockhash().unwrap();
    Transaction::new_signed_with_payer(
        instructions,
        Some(&wallet.pubkey()),
        signing_keypairs,
        latest_blockhash,
    )
}
