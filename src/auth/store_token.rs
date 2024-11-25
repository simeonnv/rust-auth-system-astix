use surrealdb::{Datetime, RecordId, Surreal};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::sql::Value;
use serde::{Serialize, Deserialize};
use crate::DB;
use crate::tokens;

#[derive(Serialize, Deserialize)]
struct TokenRecord {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Req {
    pub role: &'static str,
    pub token: String,
}

pub async fn store_token(token: &str, user_id: &str) -> surrealdb::Result<()> {
    let record = TokenRecord {
        token: token.to_string(),
    };
    let mut _res = DB
    .query(r#"

        LET $tokenRes = (CREATE tokens SET
            createdAt = time::now(),
            role = "user",
            token = $Token
        );
            
        RELATE $userId->hasToken->$tokenRes SET createdAt = time::now();
            
        SELECT username FROM $userId;

    "#)
    .bind(("Token", token.to_string()))
    .bind(("userId", user_id.to_string()))
    .await?;
    
    Ok(())
}


