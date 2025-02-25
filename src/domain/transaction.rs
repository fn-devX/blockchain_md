use secp256k1::{Secp256k1, SecretKey, Message, Signature};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use hex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: String,
}
