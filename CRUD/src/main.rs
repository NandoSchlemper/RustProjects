mod db;
mod web;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result};
use db::connection::db;

#[get("/")]
async fn index() -> impl Responder {
    "Hello! ;)"
}

#[get("/list_tables")]
async fn list_of_tables() -> Result<impl Responder> {
    let client = db().await.map_err(|e| {
        HttpResponse::InternalServerError()
            .body(format!("Falha ao conectar ao banco de dados: {}", e))
    })?;
    let table_list_query = "
    SELECT table_name
    FROM information_schema.tables
    WHERE table_schema = 'public'
    AND table_type = 'BASE TABLE'
    ";

    // Passar essas queries para o arquivo src/db/queries.rs
    let tables_list = match client.query(table_list_query, &[]).await {
        Ok(tables) => tables,
        Err(_) => {
            return Ok(
                HttpResponse::InternalServerError().body("falha na query com o banco de dados...")
            )
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Lista de tabelas: \n {:?}", tables_list)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(list_of_tables))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
