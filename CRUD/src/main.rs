mod db;
mod web;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Carregando variaveis de ambiente
    dotenv().ok();

    let db_url = env::var("DB_URL").unwrap();
    println!("URL do Banco de Dados: {:?}", db_url);

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;
    Ok(())
}
