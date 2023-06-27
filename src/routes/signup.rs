use crate::models::user::{UserInsert, UserResponse};
use crate::{schema, ConnectionPool};
use actix_web::{error, post, web, HttpResponse, Responder};
use diesel::insert_into;
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

#[derive(Serialize, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
}

#[post("/signup")]
pub async fn signup(
    pool: web::Data<ConnectionPool>,
    body: web::Json<SignUpRequest>,
) -> actix_web::Result<impl Responder> {
    use schema::users::dsl::*;

    // Retrieve a connection from the pool.
    let mut conn = pool.get().unwrap();

    // Generate cryptographically safe salt using `rand` crate.
    let mut salt_data = [0u8; 16];
    thread_rng().fill_bytes(&mut salt_data);

    // Generate hash.
    let mut hasher = Sha512::new();
    let hash_input = [body.password.as_bytes(), &salt_data].concat();
    hasher.update(hash_input);
    let hash_data = hasher.finalize();

    // Generate new User object with required values.
    let user = UserInsert {
        email: body.email.clone(),
        hash: hash_data.to_vec(),
        salt: salt_data.to_vec(),
    };

    // Add to the database.
    let user = web::block(move || {
        conn.transaction(|conn| {
            insert_into(users).values(user).execute(conn)?;
            users.select(UserResponse::as_select()).get_result(conn)
        })
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}
