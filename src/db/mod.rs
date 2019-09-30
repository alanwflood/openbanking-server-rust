use self::models::{NewUser, User};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn create_user<'a>(
    conn: &PgConnection,
    email: &'a str,
    password: &'a str,
    first_name: &'a str,
    last_name: &'a str,
) -> User {
    use self::schema::users;

    let new_user = NewUser {
        email: email,
        password: password,
        first_name: first_name,
        last_name: last_name,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error creating new user")
}
