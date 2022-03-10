use chrono::Datelike;
use serde_derive::{Deserialize, Serialize};

use serde_json::json;
use warp::{Filter};

#[derive(Serialize, Deserialize)]
struct ApplicationInfo {
    name: String,
    year: u8,
}

#[tokio::main]
async fn main() {
    let hello = warp::any().map(|| {
        let ssss = chrono::Utc::now().date().year();
        let app_info = json!({
            "name":"rust",
            "year": ssss
        });
        app_info.to_string()
    });

    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
