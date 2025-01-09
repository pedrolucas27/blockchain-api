use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH}; // Para o hash SHA256

struct Blockchain {
    chain: Vec<Block>,
    mempool: Vec<String>,
    nodes: HashSet<String>,
}

#[derive(Debug, Clone)]
struct Block {
    index: usize,
    timestamp: u64,
    transactions: Vec<String>,
    merkle_root: String,
    nonce: u64,
    previous_hash: String,
}

impl Blockchain {
    fn create_block(&mut self) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Erro ao obter o tempo")
            .as_secs();

        let merkle_root = self.generate_merkle_root(&self.mempool);

        let previous_hash = if let Some(last_block) = self.chain.last() {
            self.get_block_id(last_block)
        } else {
            "0".repeat(64)
        };

        let block = Block {
            index: self.chain.len(),
            timestamp,
            transactions: self.mempool.clone(),
            merkle_root,
            nonce: 0,
            previous_hash,
        };

        self.mempool.clear();
        self.chain.push(block.clone());

        block
    }

    fn generate_merkle_root(&self, transactions: &[String]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        transactions.join("-")
    }

    fn get_block_id(&self, block: &Block) -> String {
        format!("{}-{}-{}", block.index, block.timestamp, block.merkle_root)
    }
}

fn main() {
    let mut blockchain = Blockchain {
        chain: vec![],
        mempool: vec!["tx1".to_string(), "tx2".to_string()],
        nodes: HashSet::new(),
    };

    let new_block = blockchain.create_block();
    println!("Novo bloco criado: {:#?}", new_block);
}
