use crate::wallet::Wallet;

use solana_sdk::signature::Signer;

use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;

pub fn origin(program_id: Pubkey, data: &[u8], accounts: Vec<AccountMeta>) -> Instruction {
    Instruction::new_with_bytes(
        program_id,
        data,
        accounts,
    )
}

pub fn create_account(wallet: &Wallet, new_pubkey: &Pubkey, owner: &Pubkey, space: usize) -> Instruction {
    let minimum_blance = wallet.get_rent(space);
    system_instruction::create_account(
        &wallet.payer.pubkey(), 
        new_pubkey, 
        minimum_blance,          // 初始余额（以 lamports 为单位）
        space.try_into().unwrap(),  // 账户空间大小
        owner)
}

pub fn transfer_to(wallet: &Wallet, to: &Pubkey, lamports: u64) -> Instruction {
    system_instruction::transfer(
        &wallet.payer.pubkey(), 
        to, 
        lamports)
}