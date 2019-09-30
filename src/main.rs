mod routes;

use actix_web::{ web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use routes::{index3, index, AppState};
use std::env;

fn main() {
    dotenv().ok();
    let server_port = env::var("SERVER_PORT").unwrap_or(String::from("8080"));
    let server_url = format!("127.0.0.1:{}", server_port);

    let mut listenfd = ListenFd::from_env(); // <- Used for live reloading
    let mut server = HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix Web"),
            })
            .route("/", web::get().to(index))
            .service(index3)
    });
    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener).unwrap()
    } else {
        server.bind(server_url).unwrap()
    };
    server.run().unwrap();
}
