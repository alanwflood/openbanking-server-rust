use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};
use std::env;

pub fn send_test_mail(recipient_email_address: String) {
    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new(recipient_email_address).unwrap()),
            vec![EmailAddress::new("root@localhost".to_string()).unwrap()],
        )
        .unwrap(),
        "id1".to_string(),
        "Hello world".to_string().into_bytes(),
    );

    let mut mailer = SmtpClient::new_simple("smtp.ethereal.email")
        .unwrap()
        .credentials(Credentials::new(
            "javonte.breitenberg7@ethereal.email".to_string(),
            "mcqMDPG8Eygsd2vdSZ".to_string(),
        ))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        // Configure expected authentication mechanism
        .authentication_mechanism(Mechanism::Plain)
        // Enable connection reuse
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();
    // Send the email
    let result = mailer.send(email);

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());

    // const transporter = nodemailer.createTransport({
    // host: 'smtp.ethereal.email',
    // port: 587,
    // auth: {
    //     user: 'javonte.breitenberg7@ethereal.email',
    //     pass: 'mcqMDPG8Eygsd2vdSZ'
    // }
    // });
}
