use crate::models::error::ErrorMessage;
use crate::models::token::Token;
use crate::models::user::User;
use crate::{schema, ConnectionPool};
use actix_web::{error, post, web, HttpResponse, Responder};
use diesel::TextExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

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
    use schema::users::dsl::*;

    // Retrieve a connection from the pool.
    let mut conn = pool.get().unwrap();

    // Retrieve user from the database.
    let body_email = body.email.clone();
    let row = web::block(move || {
        users
            .filter(email.like(body_email.as_str()))
            .select(User::as_select())
            .first(&mut conn)
    })
    .await?
    .map_err(error::ErrorUnauthorized)?;

    // Generate hash.
    let mut hasher = Sha512::new();
    hasher.update([body.password.as_bytes(), &row.salt[..]].concat());
    let hash_data = hasher.finalize().to_vec();

    // Only continue if the password was correct.
    // TODO: prevent side channel attacks using cryptographically secure library.
    if !hash_data.eq(&row.hash) {
        return Ok(HttpResponse::Unauthorized().json(ErrorMessage {
            message: "Invalid Credentials".to_string(),
        }));
    }

    // Attempt to create JWT, and return if successfull.
    match Token::new(&row).encode() {
        Ok(token) => Ok(HttpResponse::Ok().body(token)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ErrorMessage {
            message: "Internal Server Error".to_string(),
        })),
    }
}
