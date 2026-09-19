use std::net::SocketAddr;

use chrono::{Datelike, Utc};
use serde_derive::{Deserialize, Serialize};
use tokio::net::TcpListener;
use viz::{serve, Request, Response, ResponseExt, Result, Router};

#[derive(Serialize, Deserialize)]
struct AppInfo {
    name: String,
    year: i32,
}

async fn hello(_: Request) -> Result<Response> {
    let app_info = AppInfo {
        name: String::from("rust-viz"),
        year: Utc::now().year(),
    };
    Ok(Response::json(app_info)?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on http://{addr}");

    let app = Router::new().get("/hello", hello);

    serve(listener, app).await?;
    Ok(())
}