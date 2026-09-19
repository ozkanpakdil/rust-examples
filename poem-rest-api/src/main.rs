use chrono::{Datelike, Utc};
use poem::{get, handler, listener::TcpListener, web::Json, Route, Server};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AppInfo {
    name: String,
    year: i32,
}

#[handler]
async fn hello() -> Json<AppInfo> {
    let app_info = AppInfo {
        name: String::from("rust-poem"),
        year: Utc::now().year(),
    };
    Json(app_info)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello", get(hello));

    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}