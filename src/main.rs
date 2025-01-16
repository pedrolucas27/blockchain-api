use actix_web::{App, HttpServer};

mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Iniciando a API...");
    HttpServer::new(|| App::new().configure(routes::blockchain_routes::init_routes))
        .bind("localhost:8080")?
        .run()
        .await
}
