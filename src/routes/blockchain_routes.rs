use crate::services::blockchain_service::BlockchainService;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/chain")]
async fn get_chain(data: web::Data<BlockchainService>) -> impl Responder {
    let chain = data.get_chain();
    HttpResponse::Ok().json(chain)
}

#[post("/mine")]
async fn mine_block(data: web::Data<BlockchainService>) -> impl Responder {
    let new_block = data.mine_block();
    HttpResponse::Ok().json(new_block)
}

#[get("/transactions/mempool")]
async fn mempool() -> impl Responder {
    HttpResponse::Ok().body("Mempool")
}

#[post("/transactions/create")]
async fn create_transaction(item: web::Json<(String, String, f64, String)>) -> impl Responder {
    HttpResponse::Ok().body("Transaction Created")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chain)
        .service(mempool)
        .service(mine_block)
        .service(create_transaction);
}
