mod db;
mod web;
use tokio_postgres::Error;
use db::connection::db;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = db().await?;

    let table_list_query = "
    SELECT table_name
    FROM information_schema.tables
    WHERE table_schema = 'public'
    AND table_type = 'BASE TABLE'
    ";

    // Passar essas queries para o arquivo src/db/queries.rs
    let tables_list = client
        .query(table_list_query, &[])
        .await?;
    println!("Tables: {:?}", tables_list);

    Ok(())
}
