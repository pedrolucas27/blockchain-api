use actix_web::{get, post, web, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_redis::{redis::Commands, RedisConnectionManager};
use serde_json;

use crate::{models::blockchain::Block, services::blockchain_service::BlockchainService};

#[get("/chain")]
async fn get_chain(service: web::Data<BlockchainService>) -> impl Responder {
    let chain = service.get_chain();
    HttpResponse::Ok().json(chain)
}

#[post("/mine")]
async fn mine_block(service: web::Data<BlockchainService>) -> impl Responder {
    let new_block = service.init_genesis_block();
    HttpResponse::Ok().json(new_block)
}

#[get("/transactions/mempool")]
async fn mempool(service: web::Data<BlockchainService>) -> impl Responder {
    let mempool: Vec<Transaction> = service.get_mempool();
    HttpResponse::Ok().body(mempool)
}

#[post("/transactions/create")]
async fn create_transaction(
    item: web::Json<(String, String, f64, String)>,
    service: web::Data<BlockchainService>,
) -> impl Responder {
    let mut request = Transaction {
        sender: item.0.clone().to_string(),
        recipient: item.1.clone().to_string(),
        amount: item.2,
        timestamp: item.3.clone().to_string(),
        signature: None,
    };

    let transaction = service.save_transaction(request);
    HttpResponse::Ok().json(transaction)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chain)
        .service(mempool)
        .service(mine_block)
        .service(create_transaction);
}
