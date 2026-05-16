use dotenvy::dotenv;
use mongodb::{Client, Database};
use std::env;

pub async fn database() -> Result<Database, mongodb::error::Error> {
    dotenv().ok();

    let uri = env::var("DATABASE_URL").expect("Var: DATABASE_URL. Não encontrada");
    let db_name = env::var("DB_NAME").expect("Var: DB_NAME. Não encontrada");
    let client = Client::with_uri_str(&uri).await?;

    Ok(client.database(&db_name))
}
