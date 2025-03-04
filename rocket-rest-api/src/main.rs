use chrono::{Datelike, Utc};
use rocket::data::Limits;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize};
use crate::rocket::data::ToByteUnit;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct AppInfo {
    name: String,
    year: i32,
}

#[get("/hello")]
async fn hello() -> Json<AppInfo> {
    let app_info = AppInfo {
        name: String::from("rust-rocket"),
        year: Utc::now().year(),
    };
    Json(app_info)
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("workers", 16))
        .merge(("port", 8080))
        .merge(("limits", Limits::new().limit("json", 2_i32.mebibytes())));

    rocket::custom(figment)
    .mount("/", routes![hello])
}
