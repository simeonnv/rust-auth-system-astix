use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use crate::config;

pub async fn init_db(db: &Surreal<Client>) -> Result<String, Error> {
    println!("Connecting to the database...");
    db.connect::<Ws>(format!("127.0.0.1:8888")).await?;
    println!("Connected!");

    println!("Signing in to the database...");
    db.signin(Root {
        username: config::USERNAME,
        password: config::PASSWORD,
    })
    .await?;
    println!("Signed in!");

    println!("Setting up the namespace and database...");
    db.use_ns(config::NAMESPACE).use_db(config::DATABASE).await?;
    println!("Setup complete!");

    Ok(String::from("RIZZ"))
}