use crate::models::error::ErrorMessage;

use crate::models::user::User;
use crate::{schema, ConnectionPool};
use actix_web::{error, post, web, HttpResponse, Responder};
use diesel::TextExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    pub email: String,
    pub password: String,
}

#[post("/token")]
pub async fn token(
    pool: web::Data<ConnectionPool>,
    body: web::Json<TokenRequest>,
) -> actix_web::Result<impl Responder> {
    // Retrieve a connection from the pool.
    let mut conn = pool.get().unwrap();

    // Retrieve user from the database.
    let email = body.email.clone();
    let user = web::block(move || User::from_email(&email, &mut conn))
        .await?
        .map_err(error::ErrorUnauthorized)?;

    // Validate the users password.
    if !user.verify(&body.password) {
        return Ok(HttpResponse::Unauthorized().json(ErrorMessage {
            message: "Invalid Credentials".to_string(),
        }));
    }

    // Attempt to create JWT, and return if successful.
    match user.token().encode() {
        Ok(token) => Ok(HttpResponse::Ok().body(token)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ErrorMessage {
            message: "Internal Server Error".to_string(),
        })),
    }
}
