use crate::db::models::User;
use crate::errors::ServiceError;
use actix_web::client::Client;
use actix_web::web;
use base64;
use futures::Future;
use serde::{Deserialize, Serialize};
use std::env;

static YAPILY_URL: &str = "https://api.yapily.com";

lazy_static::lazy_static! {
    pub static ref AUTH_TOKEN: String = {
        let app_key =
            env::var("YAPILY_APP_KEY").unwrap_or_else(|err| "MISSING APP KEY".to_string());
        let secret_key =
            env::var("YAPILY_APP_SECRET").unwrap_or_else(|err| "MISSING SECRET KEY".to_string());
        dbg!(&app_key);
        dbg!(&secret_key);

        format!(
            "Basic {}",
            base64::encode(&[app_key, ":".to_string(), secret_key].concat())
        )
    };
}

#[derive(Debug, Serialize)]
struct CreateUserBody {
    #[serde(rename(serialize = "applicationUserId"))]
    user_id: String,
    #[serde(rename(serialize = "referenceId"))]
    reference_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateUserResponse {
    pub uuid: String,
    application_uuid: String,
    application_user_id: String,
    reference_id: String,
}

pub fn create_user(
    user: &User,
    client: web::Data<Client>,
) -> impl Future<Item = String, Error = ServiceError> {
    let payload = CreateUserBody {
        user_id: user.id.to_string(),
        reference_id: user.email.clone(),
    };

    client
        .post(format!("{}{}", YAPILY_URL, "/users"))
        .header("Authorization", AUTH_TOKEN.as_str())
        .send_json(&payload)
        .map_err(|err| {
            dbg!("Auth Err: ", err);
            ServiceError::InternalServerError
        })
        .and_then(|mut resp| {
            resp.json()
                .and_then(|body: CreateUserResponse| Ok(body.uuid))
                .map_err(|err| {
                    dbg!("Something else: ", err);
                    ServiceError::InternalServerError
                })
        })
        .map_err(|err| {
            dbg!("Auth Err: ", err);
            ServiceError::InternalServerError
        })
}
