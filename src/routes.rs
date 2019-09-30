use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub struct AppState {
    pub app_name: String,
}

pub fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}
