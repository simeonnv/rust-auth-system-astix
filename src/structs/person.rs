use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    title: String,
    name: String,
    marketing: bool,
}