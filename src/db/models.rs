use super::schema::*;
use crate::db::UserData;

use actix_web::HttpResponse;
use argonautica::{Hasher, Verifier};
use chrono;
use serde_derive::Serialize;
use uuid;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String =
        std::env::var("SECRET_KEY").unwrap_or_else(|_| "1234".repeat(8));
}

pub fn hash_password(password: &str) -> Result<String, HttpResponse> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            HttpResponse::Unauthorized().json("Unauthorized")
        })
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, HttpResponse> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            HttpResponse::Unauthorized().json("Unauthorized")
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
}

impl User {
    pub fn from_user_data(user_data: UserData) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            email: user_data.email,
            hash: hash_password(&user_data.password.to_owned()).expect("Error creating new uesr"),
            first_name: user_data.first_name,
            last_name: user_data.last_name,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
