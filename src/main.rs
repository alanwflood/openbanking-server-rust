#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate lazy_static;

#[macro_use]
mod errors;
mod db;
mod routes;
mod yapily;

use crate::db::establish_connection_pool;
use actix_redis::RedisSession;
use actix_web::{client::Client, middleware, web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

fn get_server_url() -> String {
    dotenv().ok();
    let server_port = env::var("SERVER_PORT").unwrap_or("8080".to_string());
    let domain: String = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    format!("{}:{}", domain, server_port)
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let pool = establish_connection_pool(); // Create Database connection pool
    let mut listenfd = ListenFd::from_env(); // Used for live reloading

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(Client::new())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]).cookie_name("authorization"))
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api/v1/")
                    .service(routes::user_register)
                    .service(routes::user_login),
            )
    });

    // The following sets up the server for live reloading (Run watch_project.sh)
    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener).unwrap()
    } else {
        server.bind(get_server_url()).unwrap()
    };
    server.run().unwrap();
}
