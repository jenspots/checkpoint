use crate::models::token::Token;
use crate::schema::users;
use crate::ConnectionDB;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub created_at: i32,
    pub updated_at: Option<i32>,
}

impl User {
    pub fn verify(&self, password: &String) -> bool {
        let mut hasher = Sha512::new();
        let hash_input = [password.as_bytes(), &self.salt].concat();
        hasher.update(hash_input);
        hasher.finalize().to_vec().eq(&self.hash)
    }

    pub fn token(&self) -> Token {
        Token::new(self)
    }

    pub fn from_email(e: &String, conn: &mut ConnectionDB) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users
            .filter(email.like(e))
            .select(User::as_select())
            .first(conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserSignUp {
    pub email: String,
    pub password: String,
}

impl UserSignUp {
    pub fn salt() -> Vec<u8> {
        let mut result = [0u8; 16];
        thread_rng().fill_bytes(&mut result);
        result.to_vec()
    }

    pub fn hash(&self, salt: &Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha512::new();
        let hash_input = [self.password.as_bytes(), salt].concat();
        hasher.update(hash_input);
        hasher.finalize().to_vec()
    }

    pub fn insertable(&self) -> UserInsert {
        let salt = UserSignUp::salt();
        UserInsert {
            email: self.email.clone(),
            hash: self.hash(&salt),
            salt,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserInsert {
    pub email: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}

impl UserInsert {
    // TODO: This connection type should be way more generic.
    pub fn execute(&self, conn: &mut ConnectionDB) -> Result<UserResponse, Error> {
        use crate::schema::users::dsl::*;

        conn.transaction(|conn| {
            insert_into(users).values(self).execute(conn)?;
            users.select(UserResponse::as_select()).get_result(conn)
        })
    }
}

#[derive(Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserResponse {
    pub email: String,
}
