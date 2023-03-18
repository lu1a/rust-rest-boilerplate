#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use dotenv::dotenv;
use env_logger::Env;
use log::{info};
use rocket::response::content::Json;
use serde::Serialize;
use std::env;

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
    let env_type = env::var("ENVIRONMENT").unwrap_or_default();
    let response = HealthResponse { env: env_type, status: "ok".to_string() };
    let json = serde_json::to_string(&response).unwrap();
    Json(json)
}

fn main() {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    info!("This is where I'd establish db connection!");

    rocket::ignite()
        .mount("/", routes![index, health])
        .launch();
}
