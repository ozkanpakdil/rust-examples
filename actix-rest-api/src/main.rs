use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Datelike;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AppInfo {
    name: String,
    year: i32,
}

async fn index() -> impl Responder {
    let app_info = AppInfo {
        name: String::from("rust-actix"),
        year: chrono::Utc::now().date_naive().year(),
    };
    HttpResponse::Ok().json(&app_info)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hello", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
