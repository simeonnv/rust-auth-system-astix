use serde::{Serialize, Deserialize};
use surrealdb::{sql::Thing, Datetime, RecordId};

#[derive(Debug, Serialize, Deserialize)]
pub struct accounts {
    pub id: RecordId,
    pub createdAt: Datetime,
    pub password: String,
    pub pfp: Option<RecordId>,
    pub role: String,
    pub status: Option<String>,
    pub username: String
}

// pub struct accounts {
//     pub id: Thing,
//     pub createdAt: Thing,
//     pub password: Thing,
//     pub pfp: Thing,
//     pub role: Thing,
//     pub status: Thing,
//     pub username: Thing
// }