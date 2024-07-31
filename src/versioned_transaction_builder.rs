use crate::wallet::*;
use crate::send_transaction;

use solana_sdk::signature::Signer;
use solana_sdk::transaction::VersionedTransaction;

use solana_program::pubkey::Pubkey;
use solana_program::instruction::Instruction;
use solana_program::message::v0::Message as V0Message;
use solana_program::message::VersionedMessage;
use solana_program::address_lookup_table::{AddressLookupTableAccount, state::AddressLookupTable};
use solana_program::address_lookup_table::instruction::{create_lookup_table, extend_lookup_table};

pub fn create_lookup(wallet: &Wallet, new_addresses: Vec<Pubkey>) -> Pubkey {
    let slot = wallet.client.get_slot().unwrap();
    println!("slot: {}", slot);
    let (create_instruction, lookup_table_key) = create_lookup_table(
        wallet.payer.pubkey(),
        wallet.payer.pubkey(),
        slot
    );

    let extend_instruction = extend_lookup_table(
        lookup_table_key, 
        wallet.payer.pubkey(), 
        Some(wallet.payer.pubkey()), 
        new_addresses);

    // V0Message
    let blockhash = wallet.client.get_latest_blockhash().unwrap();
    let v0message = V0Message::try_compile(
        &wallet.payer.pubkey(),
        &[create_instruction, extend_instruction], 
        &[], 
        blockhash).unwrap();
    
    // VersionedMessage
    let version_message = VersionedMessage::V0(v0message);

    // Versioned_transaction
    let versioned_transaction = VersionedTransaction::try_new(version_message, &[&wallet.payer]).unwrap();

    // 发送交易
    send_transaction(wallet, &versioned_transaction);

    lookup_table_key
}

pub fn versioned_transaction(wallet: &Wallet, lookup_table_key: Pubkey, instructions: &[Instruction]) -> VersionedTransaction {

    // AddressLookupTableAccount
    let raw_account = wallet.client.get_account(&lookup_table_key).unwrap();
    let address_lookup_table = AddressLookupTable::deserialize(&raw_account.data).unwrap();
    let address_lookup_table_account = AddressLookupTableAccount {
        key: lookup_table_key,
        addresses: address_lookup_table.addresses.to_vec(),
    };

    // V0Message
    let blockhash = wallet.client.get_latest_blockhash().unwrap();
    let v0message = V0Message::try_compile(
        &wallet.payer.pubkey(),
        instructions, 
        &[address_lookup_table_account], 
        blockhash).unwrap();
    
    // VersionedMessage
    let version_message = VersionedMessage::V0(v0message);

    // Versioned_transaction
    VersionedTransaction::try_new(version_message, &[&wallet.payer]).unwrap()
}