use super::schema::*;
use crate::db::{Pool, UserData};
use crate::errors;

use actix_web::web;
use argonautica::{Hasher, Verifier};
use chrono;
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

#[derive(Debug, Queryable, Insertable, Serialize)]
#[table_name = "users"]
pub struct User {
    #[serde(skip_serializing)]
    pub id: uuid::Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip_serializing)]
    pub yapily_id: String,
}

impl User {
    pub fn from_user_data(user_data: UserData) -> Self {
        let id = uuid::Uuid::new_v4();

        User {
            id: id,
            email: user_data.email,
            hash: hash_password(&user_data.password.to_owned()).expect("Error creating new user"),
            first_name: user_data.first_name,
            last_name: user_data.last_name,
            created_at: chrono::Local::now().naive_local(),
            yapily_id: "".to_string(),
        }
    }

    pub fn set_yapily_id(&self, yapily_id: String, pool: web::Data<Pool>) -> Self {
        crate::db::set_yapily_id(self, yapily_id, pool).unwrap()
    }
}
