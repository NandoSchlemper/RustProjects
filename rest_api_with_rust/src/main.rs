use actix_web::{HttpServer, web, App, Responder};
use serde::{Serialize, Deserialize};

use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
    // Definimos Arc para garantir a compartibilidade da variavel entre as threads
    // Definimos como Mutex para garantir exclusão mutua, ou seja, apenas 1 thread pode alterar uma variavel
    // por vez, impedindo sobrescrita, inclusões e perca de informação ;)
) -> impl Responder {
    let new_user = user.into_inner();

    // LOCK para obter acesso exclusivo ao conteúdo dentro do Mutex
    // O lock nos retorna um RESULT que quando desembrulhado (UNWRAP), que BEM SUCEDIDO, retorna um MutexGuard
    let mut users = users.lock().unwrap();
    // O MutexGuard garante que a unica thread que pode acessar o valor é a que está no escopo
    users.user_list.push(new_user.clone());
    // Aqui, modificamos o valor da MutexGuard
    format!("Usuário {:?} cadastrado!", new_user.name)
    // Aqui o escopo de MutexGuard termina, ou seja, não podemos mais acessar a variavel através dessa thread
}

async fn get_all_users(users: web::Data<Arc<Mutex<Users>>>) -> impl Responder {
    let users = users.lock().unwrap();

    format!("Lista de todos os usuários: \n {:?}", users.user_list)
}


async fn delete_user(
    user: web::Json<User>,
    users: web::Data<Arc<Mutex<Users>>>
    ) -> impl Responder {
    
    let mut users = users.lock().unwrap();

    let user_find = user.into_inner();

    // Definimos posição (pos) como se fosse a busca pelo usuário dentro do array através da iteração do mesmo.
    if let Some(pos) = users.user_list.iter().position(|x| *x == user_find) {
        users.user_list.remove(pos);
        // Utilizamos o Remove, para removermos o index que foi achado pelo iter().position()
        format!("Usuário: {:?}, deletado com sucesso!", user_find.name)
    } else {
        "Usuário não encontrado...".to_string()
    }
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
            .route("/delete", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}