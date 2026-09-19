use chrono::{Datelike, Utc};
use gotham::{
    helpers::http::{response::create_response, Body},
    http::{Response, StatusCode},
    mime,
    router::builder::*,
    state::State,
};
use serde_derive::Serialize;

#[derive(Serialize)]
struct AppInfo {
    name: String,
    year: i32,
}

fn hello(state: State) -> (State, Response<Body>) {
    let app_info = AppInfo {
        name: String::from("rust-gotham"),
        year: Utc::now().year(),
    };
    let body = serde_json::to_string(&app_info).unwrap();

    let response = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

    (state, response)
}

fn main() {
    let addr = "0.0.0.0:8080";
    println!("Listening for requests at http://{}", addr);

    let router = build_simple_router(|route| {
        route.get("/hello").to(hello);
    });

    gotham::start(addr, router).unwrap();
}