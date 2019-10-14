use crate::db::{self, models::User, ForgottenPasswordReq, UserAuthReq, UserRegisterReq};
use crate::errors::ServiceError;
use crate::mail;
use crate::yapily;
use actix_session::Session;
use actix_web::{client::Client, error::BlockingError, post, web, HttpResponse};
use futures::Future;

#[post("/login")]
pub fn user_login(
    auth_data: web::Json<UserAuthReq>,
    pool: web::Data<db::Pool>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || db::login_user(auth_data.into_inner(), pool)).then(
        move |res: Result<(User, web::Data<db::Pool>), BlockingError<ServiceError>>| match res {
            Ok((user, _pool)) => session
                .set("user.id", &user.id)
                .and_then(|_| {
                    session.renew();
                    return Ok(HttpResponse::Ok().json(&user.yapily_id));
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
    user_data: web::Json<UserRegisterReq>,
    pool: web::Data<db::Pool>,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let mut new_user = User::from_user_data(user_data.into_inner());

    web::block(move || db::create_user(new_user, pool))
        .then(|res| match res {
            Ok((user, conn)) => Ok((user, conn)),
            Err(_err) => Err(ServiceError::InternalServerError),
        })
        .and_then(|(user, pool)| update_yapily_credentials(user, pool, client))
}

#[post("/forgot")]
pub fn forgotten_password(
    request: web::Json<ForgottenPasswordReq>,
    pool: web::Data<db::Pool>,
    mailer: web::Data<mail::Pool>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || User::find_by_email(request.into_inner().email, &pool)).then(move |res| {
        dbg!(&res);
        let id = uuid::Uuid::new_v4();
        match res {
            Ok(user) => session
                .set(&id.to_string(), user.id)
                .and_then(|_| {
                    mail::send_password_reset_token(user, id, mailer)?;
                    Ok(HttpResponse::Ok().json(id))
                })
                .or_else(|_| Err(ServiceError::InternalServerError)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }
    })
}

#[post("/forgot/reset")]
pub fn reset_password<'a>(
    request: web::Json<db::ResetPasswordReq>,
    pool: web::Data<db::Pool>,
    session: Session,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let token = request.reset_token.to_string();
    let retrieved_id: Result<String, ServiceError> = session
        .get(&token)
        .unwrap()
        .ok_or(ServiceError::Unauthorized);
    web::block(move || {
        let id = retrieved_id.unwrap();
        let user_id = uuid::Uuid::parse_str(&id).unwrap();
        User::reset_password(user_id, &request.new_password, &pool)
    })
    .then(move |res| match res {
        Ok(_user) => {
            session.remove(&token);
            Ok(HttpResponse::Ok().json(true))
        }
        Err(_err) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn update_yapily_credentials(
    user: User,
    pool: web::Data<db::Pool>,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    yapily::create_user(user, client).and_then(|user| match db::update_yapily_id(&user, pool) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
