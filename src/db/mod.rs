use self::models::{verify_password, User};
use crate::errors::ServiceError;
use actix_web::web;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenv::dotenv;
use serde_derive::Deserialize;
use std::env;

pub mod models;
pub mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Setup Database Pooling
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

// JSON Payload shape for User Registration
#[derive(Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

// Create user from JSON Payload
pub fn create_user(
    user_data: UserData,
    pool: web::Data<Pool>,
) -> Result<User, diesel::result::Error> {
    use self::schema::users;
    let conn = &pool.get().unwrap();
    let user = User::from_user_data(user_data);

    diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)?;
    Ok(user)
}

#[derive(Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub fn login_user(auth_data: AuthData, pool: web::Data<Pool>) -> Result<User, ServiceError> {
    use self::schema::users::dsl::{email, users};
    let conn: &PgConnection = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify_password(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into()); // convert into slimUser
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
