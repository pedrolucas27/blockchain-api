use std::sync::Mutex;

use crate::{
    models::blockchain::{Block, Transaction, TransactionRequest},
    services::blockchain_service::BlockchainService,
};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/chain")]
async fn get_chain(service: web::Data<Mutex<BlockchainService>>) -> impl Responder {
    println!("Resgatando chain...");

    let mut service = service.lock().unwrap();
    let chain: Vec<Block> = service.get_chain();

    HttpResponse::Ok().json(chain)
}

#[post("/mine")]
async fn mine_block(service: web::Data<Mutex<BlockchainService>>) -> impl Responder {
    println!("Minerando bloco...");

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
    let mut service = service.lock().unwrap();
    let mempool: Vec<Transaction> = service.get_mempool();
    HttpResponse::Ok().json(mempool)
}

#[post("/transactions/create")]
async fn create_transaction(
    item: web::Json<TransactionRequest>,
    service: web::Data<Mutex<BlockchainService>>,
) -> impl Responder {
    let transaction_request = item.into_inner();

    let mut service = service.lock().unwrap();
    match service.save_transaction(transaction_request) {
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
