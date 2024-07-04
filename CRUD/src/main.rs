mod db;
mod web;
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello! ;)"
}

#[get("/list_tables")]
async fn list_of_tables() -> impl Responder {
    "Bruh"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(list_of_tables))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
