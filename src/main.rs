use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use once_cell::sync::Lazy;
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::engine::remote::ws::{Client};
use surrealdb::{Surreal};
use env_logger::Env;

use std::net::Ipv4Addr;
use std::sync::Mutex;

mod config;

pub mod structs{
    pub mod login_struct;
    pub mod accounts;
    pub mod tokens;
}
use structs::accounts::accounts;
use structs::tokens::tokens;


mod routes {
    pub mod test;
    pub mod signup;
    pub mod login;
}
use routes::test::test;
use routes::signup::signup;

mod db {
    pub mod init_db;
}
use db::init_db::init_db;

mod auth {
    pub mod generate_token;
    pub mod store_token;
    pub mod get_token;
}

pub static DB: Lazy<Surreal<Client>> = Lazy::new(|| Surreal::init());

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if let Err(e) = init_db(&DB).await {
        eprintln!("Failed to initialize the database: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database initialization failed"));
    }

    env_logger::init_from_env(Env::default().default_filter_or("info"));


    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %s %{User-Agent}i"))
            .wrap(Cors::permissive())
            // .service(signup)
            // .service(query_person)

    });
    
    match server.bind(format!("127.0.0.1:{}", config::PORT)) {
        Ok(server) => server.run().await,
        Err(e) => {
            eprintln!("Failed to bind to port {}: {}", config::PORT, e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Server binding failed"))
        }
    }
    
}
