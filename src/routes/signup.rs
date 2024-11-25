use core::hash;
use std::error::Error;
use apistos::{api_operation, ApiComponent};
use actix_web::{body::MessageBody, post, web, Responder, HttpResponse};
use serde::Serialize;
use crate::{auth::{generate_token::generate_token, store_token::store_token}, structs::login_struct::LoginStruct};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use crate::config::SALT;
use crate::DB;
use crate::accounts;

#[derive(Serialize)]
struct Res {
    status: String,
    token: String
}


pub(crate) async fn signup(data: web::Json<LoginStruct>) -> Result<HttpResponse, Box<dyn Error>> {

    
    println!("login: username: {}, password: {}", data.username, data.password);

    let mut _res = DB
    .query(r#"

        LET $check = (
            SELECT * FROM accounts WHERE username = $username
        );

        IF $check[0] = None {
            LET $userId = (
                CREATE accounts SET 
                    username = $username,
                    password = $password,
                    role = "user"
            )
        };

        RETURN $check[0];

    "#)
    .bind(("username", data.username.clone()))
    .bind(("password", data.password.clone()))
    .await?;
    dbg!(&_res);
    println!("gyat");

    let account_data: Option<Vec<accounts>> = _res.take(2)?;
    dbg!(&account_data);

    if account_data.is_none() {
        return Ok(HttpResponse::BadRequest().json(Res {
            status: "error".to_string(),
            token: "Account already exists".to_string(),
        }));
    }


    
    if let Some(accounts_list) = account_data {
        if let Some(first_account) = accounts_list.get(0) {
            let user_id = first_account.id.to_string();
            match generate_token(&user_id) {
                Ok(token) => {
                    if let Err(e) = store_token(&token, &user_id).await {
                        return Ok(HttpResponse::InternalServerError().json(Res {
                            status: "error".to_string(),
                            token: format!("Failed to store token: {}", e),
                        }));
                    }
                    
                    return Ok(HttpResponse::Ok().json(Res {
                        status: "success".to_string(),
                        token,
                    }));
                }
                Err(e) => {
                    return Ok(HttpResponse::InternalServerError().json(Res {
                        status: "error".to_string(),
                        token: format!("Failed to generate token: {}", e),
                    }));
                }
            }
        } else {
            return Ok(HttpResponse::BadRequest().json(Res {
                status: "error".to_string(),
                token: "No account found".to_string(),
            }));
        }
    } else {
        return Ok(HttpResponse::BadRequest().json(Res {
            status: "error".to_string(),
            token: "Invalid account data".to_string(),
        }));
    }

}