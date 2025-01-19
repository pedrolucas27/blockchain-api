use std::sync::Mutex;

use crate::{
    models::blockchain::{Block, Transaction},
    services::blockchain_service::BlockchainService,
};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/chain")]
async fn get_chain(service: web::Data<Mutex<BlockchainService>>) -> impl Responder {
    println!("Processando chain...");

    let service = service.lock().unwrap();
    let chain: Vec<Block> = service.current_blockchain.chain.clone();
    HttpResponse::Ok().json(chain)
}

#[post("/mine")]
async fn mine_block(service: web::Data<Mutex<BlockchainService>>) -> impl Responder {
    let mut service = service.lock().unwrap();
    match service.mine() {
        Ok(new_block) => HttpResponse::Ok().json(new_block),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Error mining block: {}", err))
        }
    }
}

#[get("/transactions/mempool")]
async fn mempool(service: web::Data<Mutex<BlockchainService>>) -> impl Responder {
    let service = service.lock().unwrap();
    let mempool: Vec<Transaction> = service.current_blockchain.mempool.clone();
    HttpResponse::Ok().json(mempool)
}

#[post("/transactions/create")]
async fn create_transaction(
    item: web::Json<Transaction>,
    service: web::Data<Mutex<BlockchainService>>,
) -> impl Responder {
    let mut new_transaction = item.into_inner();
    let wif_key = "L1US57sChKZeyXrev9q7tFm2dgA2ktJe2NP3xzXRv6wizom5MN1U";

    let mut service = service.lock().unwrap();
    match service.save_transaction(&mut new_transaction, wif_key) {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Error saving transaction: {}", err))
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chain)
        .service(mempool)
        .service(mine_block)
        .service(create_transaction);
}
