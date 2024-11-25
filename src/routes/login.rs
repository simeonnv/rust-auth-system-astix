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
    token: String,
}

pub(crate) async fn login(data: web::Json<LoginStruct>) -> Result<HttpResponse, Box<dyn Error>> {
    println!("login: username: {}, password: {}", data.username, data.password);

    // Query the database
    let mut _res = DB
        .query(
            r#"
            LET $check = (
                SELECT * FROM accounts WHERE username = $username
            );

            IF $check[0] != None {
                RETURN crypto::argon2::compare($check[0].password, $password);
            };

            RETURN $check[0];
        "#,
        )
        .bind(("username", data.username.clone()))
        .bind(("password", data.password.clone()))
        .await?;

    dbg!(&_res);

    let account_data: Option<Vec<accounts>> = _res.take(1)?;
    let correct_password: Option<bool> = _res.take(2)?;

    dbg!(&account_data);

    if account_data.is_none() {
        return Ok(HttpResponse::BadRequest().json(Res {
            status: "error".to_string(),
            token: "Account doesn't exist".to_string(),
        }));
    }

    if let Some(false) = correct_password {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "error".to_string(),
            token: "Invalid password".to_string(),
        }));
    }

    if correct_password.is_none() {
        return Ok(HttpResponse::InternalServerError().json(Res {
            status: "error".to_string(),
            token: "Password verification failed".to_string(),
        }));
    }

    if let Some(account) = account_data {
        let user_id = account[0].id.to_string();

        // Generate and store the token
        let token = generate_token(&user_id)?;
        store_token(&token, &user_id).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "success".to_string(),
            token,
        }));
    }

    Ok(HttpResponse::InternalServerError().json(Res {
        status: "error".to_string(),
        token: "Unexpected error occurred".to_string(),
    }))
}
