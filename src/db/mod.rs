use self::models::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[macro_use]
pub mod models;
#[macro_use]
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
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
