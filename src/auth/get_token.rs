use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::sql::Value;
use serde::{Serialize, Deserialize};
use crate::tokens;

pub async fn get_token(db: &Surreal<Client>, token_id: &str) -> surrealdb::Result<Option<tokens>> {
    let result = db.select(("token", token_id)).await?;
    Ok(result)
}