use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 60 * 60;
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(config::TOKEN_SECRET.as_ref()))
}
