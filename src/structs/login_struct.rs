use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginStruct {
    pub username: String,
    pub password: String,
}