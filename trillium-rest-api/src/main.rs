use chrono::{Datelike, Utc};
use serde_derive::{Deserialize, Serialize};
use trillium::{Conn, KnownHeaderName};
use trillium_router::Router;

#[derive(Serialize, Deserialize)]
struct AppInfo {
    name: String,
    year: i32,
}

async fn hello(conn: Conn) -> Conn {
    let app_info = AppInfo {
        name: String::from("rust-trillium"),
        year: Utc::now().year(),
    };
    let body = serde_json::to_string(&app_info).expect("failed to serialize AppInfo");

    conn.with_response_header(KnownHeaderName::ContentType, "application/json")
        .ok(body)
}

fn main() {
    trillium_tokio::config()
        .with_host("0.0.0.0")
        .with_port(8080)
        .run(Router::new().get("/hello", hello));
}