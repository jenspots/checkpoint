use crate::models::user::User;
use chrono::format::Numeric::Hour;
use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Token {
    exp: i64,    // Expiration
    iat: i64,    // Issued at
    iss: String, // Issuer
    nbf: i64,    // Not before
    sub: String, // Subject
    aud: String, // Audience
    jti: Uuid,   // Token identifier
}

impl Token {
    pub fn new(user: &User) -> Token {
        Token {
            iss: "localhost:8080".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp(),
            aud: "*".to_string(),
            iat: Utc::now().timestamp(),
            nbf: Utc::now().timestamp(),
            jti: Uuid::new_v4(),
            sub: user.email.clone(),
        }
    }

    pub fn encode(&self) -> Result<String, Error> {
        jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret("secret".as_ref()),
        )
    }
}
