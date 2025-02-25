use crate::domain::transaction::Transaction;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String, difficulty: u8) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let (nonce, hash) = Self::proof_of_work(index, timestamp, &transactions, &previous_hash, difficulty);
        Self { index, timestamp, transactions, previous_hash, nonce, hash }
    }

    fn calculate_hash(index: u64, timestamp: u128, transactions: &Vec<Transaction>, previous_hash: &str, nonce: u64) -> String {
        let input = format!("{}{}{:?}{}{}", index, timestamp, transactions, previous_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    fn proof_of_work(index: u64, timestamp: u128, transactions: &Vec<Transaction>, previous_hash: &str, difficulty: u8) -> (u64, String) {
        let mut nonce = 0;
        let prefix = "0".repeat(difficulty as usize);
        loop {
            let hash = Self::calculate_hash(index, timestamp, transactions, previous_hash, nonce);
            if hash.starts_with(&prefix) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}
