#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
mod db;
mod routes;

use actix_web::{middleware, web, App, HttpServer};
use db::establish_connection_pool;
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
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(routes::index)
    });
    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener).unwrap()
    } else {
        server.bind(get_server_url()).unwrap()
    };
    server.run().unwrap();
}
