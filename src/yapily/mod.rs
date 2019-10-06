use crate::errors::ServiceError;
use actix_web::client::Client;
use base64;
use futures::Future;
use serde::{Deserialize, Serialize};
use std::env;

static YAPILY_URL: &str = "https://api.yapily.com";

pub fn get_auth_token() -> String {
    let app_key = env::var("YAPILY_APP_KEY").unwrap_or_else(|err| "MISSING APP KEY".to_string());
    let secret_key =
        env::var("YAPILY_APP_SECRET").unwrap_or_else(|err| "MISSING SECRET KEY".to_string());
    dbg!(&app_key);
    dbg!(&secret_key);

    format!(
        "Basic {}",
        base64::encode(&[app_key, ":".to_string(), secret_key].concat())
    )
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
    user_id: &uuid::Uuid,
    user_reference_id: &String,
) -> Result<String, ServiceError> {
    let client = Client::default();
    let payload = CreateUserBody {
        user_id: user_id.to_string(),
        reference_id: user_reference_id.clone(),
    };

    dbg!(&payload);

    client
        .post(format!("{}{}", YAPILY_URL, "/users")) // <- Create request builder
        .header("Authorization", get_auth_token())
        .send_json(&payload)
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError
        })
        .and_then(|mut resp| {
            dbg!(&resp);
            resp.json()
                .and_then(|body: CreateUserResponse| {
                    println!("==== JSON RESPONSE ====");
                    println!("{:?}", body);
                    Ok(body.uuid)
                })
                .map_err(|err| {
                    dbg!(err);
                    ServiceError::InternalServerError
                })
        })
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError
        })
        .wait()
}
