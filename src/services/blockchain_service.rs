use crate::models::blockchain::{Block, Blockchain};
use std::sync::Mutex;

pub struct BlockchainService {
    pub blockchain: Mutex<Blockchain>,
}

impl BlockchainService {
    pub fn new() -> Self {
        Self {
            blockchain: Mutex::new(Blockchain::new()),
        }
    }

    pub fn get_chain(&self) -> Vec<Block> {
        let blockchain = self.blockchain.lock().unwrap();
        blockchain.chain.clone()
    }

    pub fn mine_block(&self) -> Block {
        let mut blockchain = self.blockchain.lock().unwrap();
        blockchain.create_block()
    }
}
