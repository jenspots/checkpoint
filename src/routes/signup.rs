use crate::models::user::UserSignUp;
use crate::ConnectionPool;
use actix_web::{error, post, web, HttpResponse, Responder};

#[post("/signup")]
pub async fn signup(
    pool: web::Data<ConnectionPool>,
    body: web::Json<UserSignUp>,
) -> actix_web::Result<impl Responder> {
    // Retrieve a connection from the pool.
    let mut conn = pool.get().unwrap();

    // Add to the database.
    let user = web::block(move || body.insertable().execute(&mut conn))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    // Return the newly created user.
    Ok(HttpResponse::Ok().json(user))
}
