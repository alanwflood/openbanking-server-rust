use self::models::User;
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

pub fn establish_connection_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

#[derive(Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

pub fn create_user<'a>(
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
