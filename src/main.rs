use std::sync::Mutex;

use env_logger;
use log::info;

use actix_web::{web, App, HttpServer};
use r2d2_redis::RedisConnectionManager;
use services::blockchain_service::BlockchainService;

mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Iniciando a API...");

    let manager =
        RedisConnectionManager::new("redis://localhost:6379").expect("Failed to create manager");
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    info!("Conexão com o Redis bem-sucedida!");

    let blockchain_service = BlockchainService::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(blockchain_service.clone())))
            .configure(routes::blockchain_routes::init_routes)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
