use sled::{Db, IVec};
use serde_json;
use crate::domain::block::Block;

pub struct Database {
    db: Db,
}

impl Database {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open database");
        Self { db }
    }

    pub fn save_block(&self, block: &Block) {
        let key = block.index.to_be_bytes();
        let value = serde_json::to_vec(block).unwrap();
        self.db.insert(key, IVec::from(value)).unwrap();
    }

    pub fn get_block(&self, index: u64) -> Option<Block> {
        let key = index.to_be_bytes();
        self.db.get(key).unwrap().and_then(|val| serde_json::from_slice(&val).ok())
    }
}
