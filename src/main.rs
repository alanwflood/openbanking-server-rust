use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

struct AppState {
    app_name: String,
}

fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix Web"),
            })
            .route("/", web::get().to(index))
            .service(index3)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
