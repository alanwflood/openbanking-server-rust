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
            env::var("YAPILY_APP_KEY").unwrap();
        let secret_key =
            env::var("YAPILY_APP_SECRET").unwrap();
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
    mut user: User,
    client: web::Data<Client>,
) -> impl Future<Item = User, Error = ServiceError> {
    let payload = CreateUserBody {
        user_id: user.id.to_string(),
        reference_id: user.email.clone(),
    };

    client
        .post(format!("{}{}", YAPILY_URL, "/users"))
        .header("Authorization", AUTH_TOKEN.as_str())
        .send_json(&payload)
        .map_err(|err| {
            dbg!("Error Adding user to Yapily: ", err);
            ServiceError::InternalServerError
        })
        .and_then(|mut resp| {
            resp.json().map_err(|err| {
                dbg!("Error parsing reponse from Yapily: ", err);
                ServiceError::InternalServerError
            })
        })
        .and_then(|body: CreateUserResponse| {
            user.yapily_id = body.uuid;
            Ok(user)
        })
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConsents {
    id: String,
    user_uuid: String,
    institution_id: String,
    status: String,
    consent_token: String,
}

pub fn get_user_consents(
    user: User,
    client: web::Data<Client>,
) -> impl Future<Item = Vec<UserConsents>, Error = ServiceError> {
    client
        .get(format!(
            "{}{}{}{}",
            YAPILY_URL, "/users", user.yapily_id, "/consents"
        ))
        .header("Authorization", AUTH_TOKEN.as_str())
        .send()
        .map_err(|err| {
            dbg!("Error user getting user consents from Yapily: ", err);
            ServiceError::InternalServerError
        })
        .and_then(|mut resp| {
            resp.json().map_err(|err| {
                dbg!("Error parsing reponse from Yapily: ", err);
                ServiceError::InternalServerError
            })
        })
        .and_then(|body: Vec<UserConsents>| Ok(body))
}
