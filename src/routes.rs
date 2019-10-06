use crate::db::{create_user, login_user, models::User, AuthData, Pool, UserData};
use crate::errors::ServiceError;
use actix_session::Session;
use actix_web::{error::BlockingError, post, web, Error, HttpResponse};
use futures::Future;

pub fn user_register(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || create_user(user_data.into_inner(), pool).unwrap()).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

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
