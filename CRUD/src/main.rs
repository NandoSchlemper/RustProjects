mod db;
mod web; 
use dotenv::dotenv;
use std::env;

fn main() {
    // Carregando variaveis de ambiente
    dotenv().ok();

    let db_url = env::var("DB_URL").unwrap();
    println!("URL do Banco de Dados: {:?}", db_url);
    println!("Hello, world!");
}
