use crate::db::{create_user, login_user, models::User, AuthData, Pool, UserData};
use crate::errors::ServiceError;
use actix_session::Session;
use actix_web::{client::Client, error::BlockingError, post, web, Error, HttpResponse};
use futures::Future;

pub fn user_register(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let user = User::from_user_data(user_data.into_inner());

    web::block(move || create_user(user, pool))
        .map_err(|_| ServiceError::InternalServerError)
        .and_then(|(new_user, pool)| crate::yapily::create_user(new_user, client))
        .map_err(|_| ServiceError::InternalServerError)
        .map(|yapily_id| match user.set_yapily_id(yapily_id, pool) {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

// #[post("/regiser")]
// pub fn user_register(
//     user_data: web::Json<UserData>,
//     pool: web::Data<Pool>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     let user = User::from_user_data(user_data.into_inner())
//     web::block(move || create_user(user_data.into_inner(), pool)).then(|res| match res {
//         Ok(user) => Ok(HttpResponse::Ok().json(user)),
//         Err(_) => Ok(HttpResponse::InternalServerError().into()),
//     })
// }
//
#[post("/login")]
pub fn user_login(
    auth_data: web::Json<AuthData>,
    pool: web::Data<Pool>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || login_user(auth_data.into_inner(), pool)).then(
        move |res: Result<User, BlockingError<ServiceError>>| match res {
            Ok(user) => session
                .set("user.id", &user.id)
                .and_then(|_| {
                    session.renew();
                    return Ok(HttpResponse::Ok().json(true));
                })
                .or_else(|_| Err(ServiceError::InternalServerError)),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        },
    )
}

// pub fn yapily_test(
//     pool: web::Data<Pool>,
//     client: web::Data<Client>,
// ) -> impl Future<Item = HttpResponse, Error = ServiceError> {
//     let user = UserData {
//         email: "alanwflood@gmail.com".to_string(),
//         first_name: "Alan".to_string(),
//         last_name: "Flood".to_string(),
//         password: "Fluffykins".to_string(),
//     };
//
//     let new_user = User::from_user_data(user);
//
//     crate::yapily::create_user(new_user, client).and_then(|user| {
//         web::block(move || create_user(user, pool)).then(|res| match res {
//             Ok(user) => Ok(HttpResponse::Ok().json(user)),
//             Err(_) => Ok(HttpResponse::InternalServerError().into()),
//         })
//     })
// }
