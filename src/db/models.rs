use super::schema::*;
use crate::db::UserData;
use crate::errors;

use argonautica::{Hasher, Verifier};
use chrono;
use futures::Future;
use serde_derive::Serialize;
use uuid;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String =
        std::env::var("SECRET_KEY").unwrap_or_else(|_| "1234".repeat(8));
}

pub fn hash_password(password: &str) -> Result<String, errors::ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            errors::ServiceError::InternalServerError
        })
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, errors::ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            errors::ServiceError::Unauthorized
        })
}

#[derive(Queryable, Insertable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub yapily_id: String,
}

impl User {
    pub fn from_user_data(user_data: UserData) -> Result<Self, errors::ServiceError> {
        let id = uuid::Uuid::new_v4();
        let yapily_id =
            crate::yapily::create_user(&id, &user_data.email).expect("Error getting yapily id");

        dbg!("{}", &yapily_id);

        Ok(User {
            id: id,
            email: user_data.email,
            hash: hash_password(&user_data.password.to_owned()).expect("Error creating new user"),
            first_name: user_data.first_name,
            last_name: user_data.last_name,
            created_at: chrono::Local::now().naive_local(),
            yapily_id: yapily_id,
        })
    }
}
