use std::str::FromStr;

use solana_sdk::signature::Keypair;
use solana_sdk::signer::keypair;

use solana_program::pubkey::Pubkey;


pub fn new() -> Keypair {
    Keypair::new()
}

/// 字节数组
/// let secret_key: [u8; 64] = [
/// 174, 47, 154, 16, 202, 193, 206, 113, 199, 190, 53, 133, 169, 175, 31, 56, 222, 53, 138,
/// 189, 224, 216, 117, 173, 10, 149, 53, 45, 73, 251, 237, 246, 15, 185, 186, 82, 177, 240,
/// 148, 69, 241, 227, 167, 80, 141, 89, 240, 121, 121, 35, 172, 247, 68, 251, 226, 218, 48,
/// 63, 176, 109, 168, 89, 238, 135,
/// ];
/// let keypair =  keypair::new_from_bytes(secret_key);
pub fn new_from_bytes(bytes: &[u8]) -> Keypair {
    Keypair::from_bytes(bytes).unwrap()
}

/// json文件（字节数组）
/// let path = std::path::Path::new("/Users/kou/.config/solana/id.json");
/// let keypair =  keypair::new_frome_file(path);
pub fn new_frome_file(path: &std::path::Path) -> Keypair {
    keypair::read_keypair_file(path).unwrap()
}

/// 私钥
/// let keypair =  keypair::new_from_base58_string("5MaiiCavjCmn9Hs1o3eznqDEhRwxo7pXiAYez7keQUviUkauRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG");
pub fn new_from_base58_string(s: &str) -> Keypair {
    Keypair::from_base58_string(s)
}



pub fn pubkey_from_str(s: &str) -> Pubkey {
    Pubkey::from_str(s).unwrap()
}

/// 检查一个公钥是否有关联的私钥
pub fn pubkey_is_on_curve(s: &str) -> bool {
    Pubkey::from_str(s).unwrap().is_on_curve()
}