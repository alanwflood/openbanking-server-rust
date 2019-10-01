use super::schema::*;
use crate::db::UserData;

use argonautica::Hasher;
use chrono;
use serde_derive::Serialize;
use uuid;

fn hash_password(password: &str) -> Result<String, ()> {
    Hasher::default()
        .with_password(password)
        .with_secret_key("A Key")
        .hash()
        .map_err(|err| {
            dbg!(err);
        })
}

#[derive(Queryable, Insertable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
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
