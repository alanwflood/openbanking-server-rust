use crate::db::models::User;
use crate::errors;
use actix_web::web;
use lettre::{ClientSecurity, SmtpClient, SmtpConnectionManager, Transport};
use lettre_email::Email;
use r2d2;

pub type Pool = r2d2::Pool<SmtpConnectionManager>;

pub fn establish_mailer_pool() -> Pool {
    let client = SmtpClient::new(("localhost", 1025), ClientSecurity::None).unwrap();
    let manager = SmtpConnectionManager::new(client).unwrap();
    r2d2::Pool::builder().build(manager).unwrap()
}

pub fn send_password_reset_token(
    user: User,
    token: uuid::Uuid,
    pool: web::Data<Pool>,
) -> Result<(), errors::ServiceError> {
    let html = format!(
        "<h2>Hi<br><br>Your password reset token is: {}</h2>",
        token.to_string()
    );
    let text = format!("Hi Your password reset token is: {}", token.to_string());
    let email = Email::builder()
        // Addresses can be specified by the tuple (email, alias)
        .to((&user.email, &user.full_name()))
        // ... or by an address only
        .from("user@example.com")
        .subject("Hi, Hello world")
        .alternative(html, text)
        .build()
        .unwrap();

    let mut mailer = pool.get().unwrap();
    match mailer.send(email.into()) {
        Ok(_) => Ok(()),
        Err(_) => Err(errors::ServiceError::EmailServerError),
    }
}
