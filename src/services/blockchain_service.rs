use crate::models::blockchain::{Blockchain, Transaction};
use r2d2::Pool;
use r2d2_redis::redis::{Commands, RedisError};
use r2d2_redis::RedisConnectionManager;

pub struct BlockchainService {
    pool: Pool<RedisConnectionManager>,
    current_blockchain: Blockchain,
}

impl BlockchainService {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        BlockchainService {
            pool,
            current_blockchain: Blockchain::new(),
        }

        //Self::init_genesis_block(&Self);
    }

    pub fn init_genesis_block(&self) -> Result<(), RedisError> {
        let mut conn = self.pool.get()?;

        let chain_length: i64 = conn.llen("chain")?;
        let genesis = self.current_blockchain.create_block();

        if chain_length != 0 {
            let block_last = conn.lrange("chain", -1, -1);
            let json_block_serialized = serde_json::from_str(block_last);

            self.current_blockchain
                .mine_proof_of_work(json_block_serialized);
        } else {
            self.save_block(block)
        }

        Ok(())
    }

    pub fn save_block(&self, block: &Block) -> Result<(), RedisError> {
        let mut conn = self.pool.get()?;

        let serialized = serde_json::to_string(block).expect("Erro ao converter bloco para String");

        println!(serialized);

        conn.rpush("chain", block.clone());
        Ok(())
    }

    pub fn save_transaction(&self, transaction: &Transaction) -> Result<(), RedisError> {
        let mut conn = self.pool.get()?;

        let serialized = serde_json::to_string(transaction)
            .expect("Erro ao converter serializar a transação para String");

        if let Ok(signature) = Blockchain::sign(priv_wif_key, &tx_data) {
            // Converte a assinatura para uma string hexadecimal, por causa do derive serialize
            transaction.signature = Some(signature.to_string());
        }

        println!(serialized);

        conn.rpush("chain", transaction.clone());
        Ok(())
    }

    pub fn get_mempool(&self) -> Result<Vec<Transaction>, RedisError> {
        let mut conn = self.pool.get()?;

        let mempool: Vec<Transaction> = conn
            .lrange("mempool", 0, -1)?
            .into_iter()
            .map(|tx_str| serde_json::from_str(&tx_str).expect("Erro ao desserializar transação"))
            .collect();

        Ok(mempool)
    }

    pub fn get_chain(&self) -> Result<Vec<Block>, RedisError> {
        let mut conn = self.pool.get()?;

        let chain: Vec<Block> = conn
            .lrange("chain", 0, -1)?
            .into_iter()
            .map(|block_str| {
                serde_json::from_str(&block_str).expect("Erro ao desserializar o bloco")
            })
            .collect();

        Ok(chain)
    }
}
