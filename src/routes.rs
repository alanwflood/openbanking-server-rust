use crate::db::{create_user, login_user, models::User, AuthData, Pool, UserData};
use crate::errors::ServiceError;
use actix_session::Session;
use actix_web::{client::Client, error::BlockingError, post, web, HttpResponse};
use futures::Future;

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

#[post("/register")]
pub fn user_register(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let mut new_user = User::from_user_data(user_data.into_inner());
    let conn = pool.get().unwrap();
    let conn2 = pool.clone().get().unwrap();

    web::block(move || create_user(new_user, &conn))
        .then(|res| match res {
            Ok(user) => Ok(user),
            Err(_) => panic!("Error Creating User"),
        })
        .and_then(|user| {
            crate::yapily::create_user(&user, client).and_then(|id| match crate::db::set_yapily_id(
                &user,
                id,
                &pool.clone().get().unwrap(),
            ) {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            })
        })
}
