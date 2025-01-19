use actix_web::{App, HttpServer};
use models::blockchain::Blockchain;

mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut blockchain = Blockchain::new();

    println!("Block genesis: {:#?}", blockchain.chain.first());

    let new_block = blockchain.create_block();
    println!("Novo bloco criado: {:#?}", new_block);

    let hash = Blockchain::generate_hash("dados para hash");
    println!("Hash gerada: {}", hash);

    let block_hash = Blockchain::get_block_id(&new_block);
    println!("Hash do bloco: {}", block_hash);

    let wif_key = "L1US57sChKZeyXrev9q7tFm2dgA2ktJe2NP3xzXRv6wizom5MN1U";
    let message = "Mensagem para assinar";

    match Blockchain::sign(wif_key, message) {
        Ok(signature) => println!("Assinatura gerada: {:?}", signature),
        Err(e) => println!("Erro ao assinar: {}", e),
    }

    let transaction = blockchain.create_transaction(
        "19sXoSbfcQD9K66f5hwP5vLwsaRyKLPgXF",
        "1MxTkeEP2PmHSMze5tUZ1hAV3YTKu2Gh1N",
        100,
        "123123123",
        "L1US57sChKZeyXrev9q7tFm2dgA2ktJe2NP3xzXRv6wizom5MN1U",
    );

    println!("transação: {:#?}", transaction);

    let mut new_block = blockchain.create_block();
    Blockchain::mine_proof_of_work(&mut new_block);
    println!("Novo bloco criado de novo: {:#?}", new_block);

    // ##############################################################################################

    println!("Iniciando a API...");

    let wif_key = "L1US57sChKZeyXrev9q7tFm2dgA2ktJe2NP3xzXRv6wizom5MN1U";
    let message = "Mensagem para assinar";

    let blockchain_service = BlockchainService::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain_service.clone()))
            .configure(routes::blockchain_routes::init_routes)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
