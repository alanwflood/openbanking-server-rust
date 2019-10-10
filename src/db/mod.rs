use self::models::User;
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
pub struct UserRegisterReq {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

// Create user
pub fn create_user(
    user: User,
    pool: web::Data<Pool>,
) -> Result<(User, web::Data<Pool>), diesel::result::Error> {
    use self::schema::users;
    let conn = &pool.get().unwrap();
    diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)?;
    Ok((user, pool))
}

#[derive(Deserialize)]
pub struct UserAuthReq {
    pub email: String,
    pub password: String,
}

pub fn login_user(
    auth_data: UserAuthReq,
    pool: web::Data<Pool>,
) -> Result<(User, web::Data<Pool>), ServiceError> {
    let user = User::find_by_email(auth_data.email, &pool)?;

    if let Ok(matching) = user.verify_password(&auth_data.password) {
        if matching {
            return Ok((user.into(), pool)); // convert into slimUser
        }
    }
    Err(ServiceError::Unauthorized)
}

pub fn update_yapily_id(user: &User, pool: web::Data<Pool>) -> Result<User, diesel::result::Error> {
    use self::schema::users::dsl::{users, yapily_id};
    let conn: &PgConnection = &pool.get().unwrap();
    let user = diesel::update(users.find(user.id))
        .set(yapily_id.eq(&user.yapily_id))
        .get_result::<User>(conn)
        .unwrap();
    Ok(user)
}

#[derive(Deserialize)]
pub struct ForgottenPasswordReq {
    pub email: String,
}
