use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DBError};
use lettre::smtp::error::Error as EmailError;
use std::convert::From;
use std::fmt;
use uuid::parser::ParseError;

#[derive(Debug)]
pub enum ServiceError {
    EmailServerError,
    InternalServerError,
    BadRequest(String),
    Unauthorized,
}

// Implement Display trait for ServiceErrors
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ServiceError::EmailServerError => write!(f, "Email Server Error"),
            ServiceError::InternalServerError => write!(f, "Internal Server Error"),
            ServiceError::BadRequest(error) => write!(f, "Bad Request {}", error),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::EmailServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<EmailError> for ServiceError {
    fn from(_: EmailError) -> ServiceError {
        ServiceError::EmailServerError
    }
}
