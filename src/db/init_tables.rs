use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

pub async fn init_tables(db: &Surreal<Client>) -> Result<(), Error> {
    let res = db.query(r#"
    
        DEFINE TABLE IF NOT EXISTS accounts SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS username ON TABLE accounts TYPE string;
        DEFINE FIELD IF NOT EXISTS status ON TABLE accounts TYPE option<string> DEFAULT NONE;
        DEFINE FIELD IF NOT EXISTS password ON TABLE accounts TYPE string;
        DEFINE FIELD IF NOT EXISTS createdAt ON TABLE accounts TYPE datetime VALUE time::now() READONLY ;
        DEFINE FIELD IF NOT EXISTS pfp ON TABLE accounts TYPE option<record<files>> DEFAULT NONE;
        DEFINE FIELD IF NOT EXISTS role ON TABLE accounts TYPE string;

        DEFINE TABLE IF NOT EXISTS tokens SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS createdAt ON TABLE tokens TYPE datetime READONLY;
        DEFINE FIELD IF NOT EXISTS role ON TABLE tokens TYPE string;
        DEFINE FIELD IF NOT EXISTS token ON TABLE tokens TYPE string READONLY;

        DEFINE TABLE IF NOT EXISTS hasToken SCHEMAFULL TYPE RELATION IN accounts OUT tokens;
        DEFINE FIELD IF NOT EXISTS createdAt ON TABLE hasToken TYPE datetime READONLY;

        DEFINE TABLE IF NOT EXISTS files SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS createdAt ON TABLE files TYPE datetime VALUE time::now() READONLY;
        DEFINE FIELD IF NOT EXISTS name ON TABLE files TYPE string READONLY;
        DEFINE FIELD IF NOT EXISTS type ON TABLE files TYPE string READONLY;
        DEFINE FIELD IF NOT EXISTS size ON TABLE files TYPE number READONLY;
        DEFINE FIELD IF NOT EXISTS base64 ON TABLE files TYPE string READONLY;

        DEFINE TABLE IF NOT EXISTS groups SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS createdAt ON TABLE groups TYPE datetime READONLY;
        DEFINE FIELD IF NOT EXISTS icon ON TABLE groups TYPE option<record<files>> DEFAULT NONE;
        DEFINE FIELD IF NOT EXISTS name ON TABLE groups TYPE string;
        DEFINE FIELD IF NOT EXISTS description ON TABLE groups TYPE string;

        DEFINE TABLE IF NOT EXISTS hasMembers SCHEMAFULL TYPE RELATION IN groups OUT accounts;
        DEFINE FIELD IF NOT EXISTS joinDate ON TABLE hasMembers TYPE datetime READONLY;
        DEFINE FIELD IF NOT EXISTS role ON TABLE hasMembers TYPE string DEFAULT "user";

        DEFINE TABLE IF NOT EXISTS messages SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS createdAt ON TABLE messages TYPE datetime READONLY;
        DEFINE FIELD IF NOT EXISTS attachment ON TABLE messages TYPE option<record<files>> DEFAULT NONE;
        DEFINE FIELD IF NOT EXISTS text ON TABLE messages TYPE option<string> DEFAULT NONE;
        DEFINE FIELD IF NOT EXISTS sentBy ON TABLE messages TYPE record<accounts>;
        DEFINE FIELD IF NOT EXISTS inChannel ON TABLE messages TYPE record<textChannels> | record<voiceChannels>;
        DEFINE FIELD IF NOT EXISTS deleted ON TABLE messages TYPE bool DEFAULT FALSE;

        DEFINE TABLE IF NOT EXISTS textChannels SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS name ON TABLE textChannels TYPE string;
        DEFINE FIELD IF NOT EXISTS group ON TABLE textChannels TYPE record<groups> READONLY;
        DEFINE FIELD IF NOT EXISTS createdAt ON TABLE textChannels TYPE datetime READONLY;
        DEFINE FIELD IF NOT EXISTS role ON TABLE textChannels TYPE string DEFAULT "user";

        DEFINE TABLE IF NOT EXISTS voiceChannels SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS name ON TABLE voiceChannels TYPE string;
        DEFINE FIELD IF NOT EXISTS group ON TABLE voiceChannels TYPE record<groups> READONLY;
        DEFINE FIELD IF NOT EXISTS createdAt ON TABLE voiceChannels TYPE datetime READONLY;
        DEFINE FIELD IF NOT EXISTS role ON TABLE voiceChannels TYPE string DEFAULT "user";

    "#).await?;
    Ok(())
}