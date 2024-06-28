use actix_web::{HttpServer, web, App, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String, 
    age: i32,
}

async fn create_user(user: web::Data<User>) -> impl Responder {
    let new_user = user;
    format!("Usuário {} cadastrado!", new_user.name)
}


async fn handler_json() -> impl Responder {
    "Yay"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running server in host: 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handler_json))
            .route("/", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}