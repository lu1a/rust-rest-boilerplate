#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use dotenv::dotenv;
use rocket::response::content::Json;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    env: String,
    status: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/health")]
fn health() -> Json<String> {
    let env_type = std::env::var("ENVIRONMENT").unwrap_or_default();
    let response = HealthResponse { env: env_type, status: "ok".to_string() };
    let json = serde_json::to_string(&response).unwrap();
    Json(json)
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![health])
        .launch();
}
