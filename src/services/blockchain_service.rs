use crate::models::blockchain::{Block, Blockchain, Transaction, TransactionRequest};
use r2d2::Pool;
use r2d2_redis::redis::{Commands, RedisError};
use r2d2_redis::RedisConnectionManager;

#[derive(Clone)]
pub struct BlockchainService {
    pool: Pool<RedisConnectionManager>,
    pub current_blockchain: Blockchain,
}

impl BlockchainService {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        let mut service = BlockchainService {
            pool,
            current_blockchain: Blockchain::new(),
        };

        let _ = service.start_blockchain();

        service
    }

    pub fn start_blockchain(&mut self) -> Result<(), RedisError> {
        let mut conn = self
            .pool
            .get()
            .expect("Falha ao tentar obter conexão com redis:");

        let exists: bool = conn.exists("blockchain")?;

        if exists {
            let serialized_blockchain: String = conn
                .get("blockchain")
                .expect("Erro ao tentar recuperar blockchain");

            self.current_blockchain = serde_json::from_str(&serialized_blockchain)
                .expect("Erro ao converter blockchain para json");
        } else {
            self.persist_db();
        }

        Ok(())
    }

    pub fn get_chain(&mut self) -> Vec<Block> {
        self.update_current_blockchain();

        self.current_blockchain.chain.clone()
    }

    pub fn get_mempool(&mut self) -> Vec<Transaction> {
        self.update_current_blockchain();

        self.current_blockchain.mempool.clone()
    }

    pub fn mine(&mut self) -> Result<Block, RedisError> {
        self.update_current_blockchain();

        self.current_blockchain.create_block();

        self.current_blockchain.mine_proof_of_work();

        self.persist_db();

        if let Some(last_block) = self.current_blockchain.chain.last() {
            Ok(last_block.clone())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Blockchain vazia").into())
        }
    }

    pub fn save_transaction(
        &mut self,
        transaction_request: TransactionRequest,
    ) -> Result<Transaction, RedisError> {
        self.update_current_blockchain();

        let mut transaction = Transaction {
            sender: transaction_request.sender,
            recipient: transaction_request.recipient,
            signature: None,
            timestamp: transaction_request.timestamp,
            amount: transaction_request.amount,
        };

        let new_transaction = self
            .current_blockchain
            .create_transaction(&mut transaction, &transaction_request.priv_wif_key);
        self.persist_db();

        Ok(new_transaction)
    }

    fn persist_db(&self) {
        let mut conn = self
            .pool
            .get()
            .expect("Falha ao tentar obter conexão com redis:");

        let serialized_blockchain = serde_json::to_string(&self.current_blockchain).unwrap();
        conn.set::<&str, String, ()>("blockchain", serialized_blockchain)
            .unwrap();
    }

    pub fn update_current_blockchain(&mut self) {
        let mut conn = self
            .pool
            .get()
            .expect("Falha ao tentar obter conexão com redis:");

        let serialized_blockchain: String = conn
            .get("blockchain")
            .expect("Erro ao tentar recuperar blockchain");

        self.current_blockchain = serde_json::from_str(&serialized_blockchain)
            .expect("Erro ao converter blockchain para json");
    }
}
