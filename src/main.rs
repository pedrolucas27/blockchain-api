use actix_web::{App, HttpServer};
use models::blockchain::Blockchain;

mod models;
mod routes;
mod services;

// #[actix_web::main]
fn main() {
    // println!("Iniciando a API...");
    // HttpServer::new(|| App::new().configure(routes::blockchain_routes::init_routes))
    //     .bind("localhost:8080")?
    //     .run()
    //     .await
    let mut blockchain = Blockchain {
        chain: vec![],
        mempool: vec!["tx1".to_string(), "tx2".to_string()], 
    };
    let new_block = blockchain.create_block();
    println!("Novo bloco criado: {:#?}", new_block);

    let hash = Blockchain::generate_hash("dados para hash");
    println!("Hash gerada: {}", hash);
    
    let block_hash = Blockchain::get_block_id(&new_block);
    println!("Hash do bloco: {}", block_hash);
}
