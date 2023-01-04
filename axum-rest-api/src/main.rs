use axum::{Json, Router, routing::get};
use chrono::{Datelike, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AppInfo {
    name: String,
    year: i32,
}

async fn hello() -> Json<AppInfo> {
    let app_info = AppInfo {
        name: String::from("rust-axum"),
        year: Utc::now().year()
    };
    Json(app_info)
}


#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/hello", get(hello));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
