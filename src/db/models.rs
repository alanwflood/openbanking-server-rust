use super::schema::*;
use crate::db::{Pool, UserRegisterReq};
use crate::errors;
use crate::errors::ServiceError;

use actix_web::web;
use argonautica::{Hasher, Verifier};
use chrono;
use diesel::{prelude::*, PgConnection};
use serde_derive::Serialize;
use uuid;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String =
        std::env::var("SECRET_KEY").unwrap_or_else(|_| "1234".repeat(8));
}

fn hash_password(password: &str) -> Result<String, errors::ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            errors::ServiceError::InternalServerError
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
    pub fn from_user_data(user_data: UserRegisterReq) -> Self {
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

    pub fn find_by_email(user_email: String, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
        use super::schema::users::dsl::{email, users};
        let conn: &PgConnection = &pool.get().unwrap();
        let user = users.filter(email.eq(&user_email)).first::<User>(conn)?;
        Ok(user.into())
    }

    pub fn reset_password(
        user_id: uuid::Uuid,
        new_password: &str,
        pool: &web::Data<Pool>,
    ) -> Result<User, ServiceError> {
        use super::schema::users::dsl::{hash, users};
        let conn: &PgConnection = &pool.get().unwrap();
        let new_hash = hash_password(new_password)?;
        let user = diesel::update(users.find(user_id))
            .set(hash.eq(new_hash))
            .get_result::<User>(conn)?;
        Ok(user)
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, errors::ServiceError> {
        Verifier::default()
            .with_hash(&self.hash)
            .with_password(password)
            .with_secret_key(SECRET_KEY.as_str())
            .verify()
            .map_err(|err| {
                dbg!(err);
                errors::ServiceError::Unauthorized
            })
    }
}
