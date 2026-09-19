use salvo::prelude::*;
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct AppInfo {
    name: String,
    year: i32,
}

#[handler]
async fn hello() -> Json<AppInfo> {
    let app_info = AppInfo {
        name: String::from("rust-salvo"),
        year: Utc::now().year()
    };
    Json(app_info)
}

#[tokio::main]
async fn main() {
    // Initialize logging subsystem
    tracing_subscriber::fmt().init();

    // Bind server to port 8080
    let acceptor = TcpListener::new("0.0.0.0:8080").bind().await;

    // Create router with two endpoints:
    // - / (root path) returns English greeting
    // - /hello returns Chinese greeting
    let router = Router::new()
        .push(Router::with_path("/hello").get(hello));

    // Print router structure for debugging
    println!("{router:?}");

    // Start serving requests
    Server::new(acceptor).serve(router).await;
}
