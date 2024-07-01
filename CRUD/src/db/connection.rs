use tokio_postgres::{Client, Error, NoTls};

pub async fn db() -> Result<Client, Error>{
    let (client, connection) = 
        tokio_postgres::connect("host=localhost
                                user=grupo_do_henrique
                                dbname=CRUD_com_rust 
                                password=cucuz_de_frango_com_arroz_e_bom", NoTls).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erro na conexão com o DB >->: {}", e)
        }
    });

    println!("Conexão com o banco de dados bem sucedida! o-O");

    Ok(client)
}