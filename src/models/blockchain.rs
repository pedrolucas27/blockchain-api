use bitcoin::hashes::Hash;
use bitcoin::key::PrivateKey;
use bitcoin::secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};
use hex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
    pub nonce: u64,
    pub previous_hash: String,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
    timestamp: String,
    pub signature: Option<String>, // Por causa de problemas com o derive Serialize no Signature
                                   // signature: Option<Result<Signature, std::string::String>>,
}

const DIFFICULTY: usize = 4;

impl Blockchain {
    pub fn new() -> Self {
        //construtor
        let mut initial_blockchain = Blockchain {
            chain: Vec::new(),
            mempool: Vec::new(),
        };

        initial_blockchain.create_genesis_block();

        initial_blockchain
    }

    fn create_genesis_block(&mut self) -> Block {
        // Usado apenas no construtor. Cria, minera e retorna o bloco gênesis do blockchain
        let genesis_block = self.create_block();

        if let Some(last_block) = self.chain.last_mut() {
            Self::mine_proof_of_work(last_block);
        } else {
            println!("A chain está vazia!");
        }

        genesis_block
    }

    pub fn create_block(&mut self) -> Block {
        // Cria um novo bloco, inclui todas as transações pendentes e adiciona ao chain. O bloco ainda não tem nonce válido, deve ser minerado
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

    fn generate_merkle_root(&self, transactions: &Vec<Transaction>) -> String {
        // Retorna a Merkle Root de um conjunto de transações
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        let mut tx_hashes: Vec<String> = transactions
            .iter()
            .map(|tx| {
                let serialized =
                    serde_json::to_string(&tx).expect("Erro ao serializar a transação modificado");
                Self::generate_hash(&serialized)
            })
            .collect();

        Self::hash_tx_hashes(&mut tx_hashes)
    }

    fn hash_tx_hashes(tx_hashes: &mut Vec<String>) -> String {
        // Função auxiliar recursiva para cálculo da Merkle Root
        if tx_hashes.len() == 1 {
            return tx_hashes[0].clone();
        }

        if tx_hashes.len() % 2 != 0 {
            tx_hashes.push(tx_hashes.last().unwrap().clone());
        }

        let mut new_tx_hashes = vec![];
        for i in (0..tx_hashes.len()).step_by(2) {
            let concatenated = format!("{}{}", tx_hashes[i], tx_hashes[i + 1]);
            new_tx_hashes.push(Blockchain::generate_hash(&concatenated));
        }

        Self::hash_tx_hashes(&mut new_tx_hashes)
    }

    pub fn get_block_id(block: &Block) -> String {
        // Retorna o ID do bloco passado como argumento. O ID de um bloco é o hash do seu cabeçalho
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
        // Gera um hash a partir dos dados passados como argumento
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn is_valid_proof(block: &mut Block, nonce: u64) -> bool {
        // Verifica se o nonce é válido para o bloco. Um nonce válido é um número que, gera um hash com 'DIFFICULTY' zeros à esquerda
        block.nonce = nonce;
        let block_id = Blockchain::generate_hash(
            &serde_json::to_string(block).expect("Erro ao serializar o bloco"),
        );
        block_id.chars().take(DIFFICULTY).all(|c| c == '0')
    }

    pub fn mine_proof_of_work(block: &mut Block) -> u64 {
        // Retorna um nonce válido para o bloco passado
        let mut nonce: u64 = 0;
        while Self::is_valid_proof(block, nonce) == false {
            nonce += 1;
        }
        nonce
    }

    pub fn create_transaction(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
        timestamp: &str,
        priv_wif_key: &str,
    ) -> Transaction {
        // Cria, insere no mempool e retorna uma nova transação, assinada pela chave privada WIF do remetente.

        let mut transaction = Transaction {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
            timestamp: timestamp.to_string(),
            signature: None,
        };

        // A message enviada é o cabeçalho da transação, sem o signature
        let tx_data =
            serde_json::to_string(&transaction).expect("Erro ao serializar a transação para JSON");

        if let Ok(signature) = Blockchain::sign(priv_wif_key, &tx_data) {
            // Converte a assinatura para uma string hexadecimal, por causa do derive serialize
            transaction.signature = Some(signature.to_string());
        }

        // transaction.signature = Some(Blockchain::sign(priv_wif_key, "message"));

        self.mempool.push(transaction.clone());

        transaction
    }

    pub fn sign(wif_compressed_priv_key: &str, message: &str) -> Result<Signature, String> {
        // Retorna a assinatura digital da mensagem e a respectiva chave privada WIF-compressed
        let secp = Secp256k1::new();

        // Converte a chave privada WIF-compressed para `SecretKey`
        let private_key = PrivateKey::from_wif(wif_compressed_priv_key)
            .map_err(|e| format!("Erro ao interpretar WIF: {}", e))?;

        let secret_key = private_key.inner;

        // Gera a mensagem como um hash de 32 bytes (SHA256)
        let msg_hash = bitcoin::hashes::sha256::Hash::hash(message.as_bytes());

        let message = Message::from_digest_slice(msg_hash.as_ref())
            .map_err(|e| format!("Erro ao criar a mensagem: {}", e))?;

        // Assina a mensagem
        let signature = secp.sign_ecdsa(&message, &secret_key);

        Ok(signature)
    }

    // TODO: revisar biblioteca tokio, client
    pub async fn resolve_conflicts(&mut self, nodes: Vec<String>) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let mut candidate_chain: Vec<Block> = Vec::new();

        for api_url in nodes {
            let incoming_chain: Vec<Block> = client
                .get(&format!("{}/chain", api_url))
                .send()
                .await?
                .json()
                .await?;
            let is_bigger_than_current_chain = incoming_chain.len() > self.chain.len();
            let is_bigger_than_candidate_chain = incoming_chain.len() > candidate_chain.len();
            let is_valid_chain = self.is_valid_chain(&incoming_chain);

            if is_bigger_than_current_chain && is_valid_chain && is_bigger_than_candidate_chain {
                candidate_chain = incoming_chain;
            }
        }

        if !candidate_chain.is_empty() {
            self.chain = candidate_chain;
        }

        Ok(())
    }

    pub fn is_valid_chain(&self, chain: &Vec<Block>) -> bool {
        for i in (0..chain.len()).rev() {
            let block = &chain[i];
            let previous_block = if i > 0 { &chain[i - 1] } else { block };
            let is_genesis = i == 0;
            let mut valid_transaction = true;

            let mut modified_block = block.clone();
            if !Self::is_valid_proof(&mut modified_block, block.nonce) {
                return false;
            }

            for transaction in &block.transactions {
                let mut transaction_copy = transaction.clone();
                let signature = transaction_copy.signature.take();

                if let Some(signature) = signature {
                    let message = serde_json::to_string(&transaction_copy).unwrap();
                    if !Self::verify_signature(&transaction.sender, &signature, &message) {
                        valid_transaction = false;
                    }
                } else {
                    valid_transaction = false;
                }
            }

            if !valid_transaction {
                return false;
            }

            if self.generate_merkle_root(&block.transactions) != block.merkle_root {
                return false;
            }

            if !self.verify_is_valid_previous_hash(block, previous_block, is_genesis) {
                return false;
            }
        }

        true
    }

    pub fn verify_signature(address: &str, signature: &str, message: &str) -> bool {
        let secp = Secp256k1::verification_only();
        let public_key = match PublicKey::from_slice(&hex::decode(address).unwrap_or_default()) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        let signature = match Signature::from_str(signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        let message_hash = bitcoin::hashes::sha256::Hash::hash(message.as_bytes());
        let message = match Message::from_digest_slice(message_hash.as_ref()) {
            Ok(msg) => msg,
            Err(_) => return false,
        };

        secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
    }

    pub fn verify_is_valid_previous_hash(
        &self,
        block: &Block,
        previous_block: &Block,
        is_genesis: bool,
    ) -> bool {
        if is_genesis && block.previous_hash == "0".repeat(64) {
            return true;
        }

        if block.previous_hash == Self::get_block_id(previous_block) {
            return true;
        }

        false
    }
}
