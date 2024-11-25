use actix_web::{get, Responder};

#[get("/")]
pub async fn test() -> impl Responder {
    "sigma"
}