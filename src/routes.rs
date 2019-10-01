use crate::db::{create_user, Pool, UserData};
use actix_web::{post, web, Error, HttpResponse};
use futures::Future;

#[post("/")]
pub fn index(
    payload: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || create_user(payload.into_inner(), pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
