use actix_web::{HttpServer, web, App, Responder};
use serde::{Serialize, Deserialize};

use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct User {
    name: String, 
    age: i32,
}

#[derive(Serialize, Deserialize)]
struct Users {
    user_list: Vec<User>
}

async fn create_user(
    user: web::Json<User>, 
    users: web::Data<Arc<Mutex<Users>>>
) -> impl Responder {
    let new_user = user.into_inner();

    // Utilizamos lock e unwrap para acessar a estrutura dentro do Mutex de forma segura
    // Criamos um atributo chamado "guards"
    let mut users = users.lock().unwrap();
    users.user_list.push(new_user.clone());
    format!("Usuário {:?} cadastrado!", new_user.name)
}

async fn get_all_users(users: web::Data<Arc<Mutex<Users>>>) -> impl Responder {
    let users = users.lock().unwrap();

    format!("Lista de todos os usuários: \n {:?}", users.user_list)
}

async fn handler_json() -> impl Responder {
    "Yay"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Definindo variaveis desse escopo

    // Instanciando a lista de usuários
    let shared_user_list = web::Data::new(Arc::new(Mutex::new(Users { user_list: vec![] })));

    println!("Running server in host: 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(shared_user_list.clone())
            .route("/", web::get().to(handler_json))
            .route("/", web::post().to(create_user))
            .route("/get_users", web::get().to(get_all_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}