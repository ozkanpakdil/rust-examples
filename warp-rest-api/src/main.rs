use chrono::Datelike;
use serde_derive::{Deserialize, Serialize};

use warp::Filter;

#[derive(Serialize, Deserialize)]
struct ApplicationInfo {
    name: String,
    year: i32,
}

#[tokio::main]
async fn main() {
    let hello = warp::any().map(|| {
        let app_info = ApplicationInfo {
            name: String::from("rust-warp"),
            year: chrono::Utc::now().date().year(),
        };
        warp::reply::json(&app_info)
    });

    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
