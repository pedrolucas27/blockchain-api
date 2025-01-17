use serde::Serialize;
use serde_json::{self, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<String>,
    pub merkle_root: String,
    pub nonce: u64,
    pub previous_hash: String,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<String>,
    pub nodes: HashSet<String>,
}

const DIFFICULTY: usize = 4;

impl Blockchain {
    pub fn create_block(&mut self) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Erro ao obter o tempo")
            .as_secs();

        let merkle_root = self.generate_merkle_root(&self.mempool);

        let previous_hash = if let Some(last_block) = self.chain.last() {
            Self::get_block_id(last_block)
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

    pub fn get_block_id(block: &Block) -> String {
        let mut block_copy =
            serde_json::to_value(block).expect("Erro ao converter bloco para JSON");
        if let Value::Object(ref mut map) = block_copy {
            map.remove("transactions");
        }
        let serialized =
            serde_json::to_string(&block_copy).expect("Erro ao serializar o bloco modificado");
        Self::generate_hash(&serialized)
    }

    pub fn generate_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn is_valid_proof(block: &mut Block, nonce: u64) -> bool {
        block.nonce = nonce;
        let block_id = Blockchain::generate_hash(
            &serde_json::to_string(block).expect("Erro ao serializar o bloco"),
        );
        block_id.chars().take(DIFFICULTY).all(|c| c == '0')
    }

    pub fn mine_proof_of_work(&self, block: &mut Block) -> u64{
        // Retorna um nonce válido para o bloco passado
        let mut nonce: u64 = 0;
        while Self::is_valid_proof(block, nonce) == false {
            nonce += 1;
        }
        nonce
    }
}
