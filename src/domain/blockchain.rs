use crate::modules::block::Block;
use crate::modules::transaction::Transaction;
use crate::modules::database::Database;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub db: Database,
}

impl Blockchain {
    pub fn new() -> Self {
        let db = Database::new();
        let chain = db.load_chain().unwrap_or_else(|| vec![Block::new(0, vec![], "0".to_string())]);
        Blockchain { chain, pending_transactions: vec![], db }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.pending_transactions.push(tx);
    }

    pub fn mine_block(&mut self) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(last_block.index + 1, self.pending_transactions.clone(), last_block.hash.clone());
        self.db.save_block(&new_block);
        self.chain.push(new_block);
        self.pending_transactions.clear();
    }
}
