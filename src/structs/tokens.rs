use serde::{Serialize, Deserialize};
use surrealdb::{sql::Thing, Datetime, RecordId};

#[derive(Debug, Serialize, Deserialize)]
pub struct tokens {
    pub id: RecordId,
    pub createdAt: Datetime,
    pub role: String,
    pub token: String
}
