use tokio_postgres::{Client, Error, NoTls};

// =postgres://Fernando:Rfm*102030@db:5432/crud

pub async fn db() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=db
                user=Fernando
                dbname=crud 
                password=Rfm*102030",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erro na conexão com o DB >->: {}", e)
        }
    });

    println!("Conexão com o banco de dados bem sucedida! o-O");

    Ok(client)
}
