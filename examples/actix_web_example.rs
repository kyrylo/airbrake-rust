extern crate log;

use std::panic;
use actix_web::{get, web, App, HttpServer, Responder};
use airbrake::*;

#[get("/divide/{numerator}/{denominator}")]
async fn divider(info: web::Path<(u32, u32)>) -> impl Responder {
    let result = info.0 / info.1;
    format!("{} / {} = {}", info.1, info.0, result)
}

#[actix_rt::main]
async fn actix_main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(divider))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn main() -> () {
    env_logger::init();

    // Set up the client
    let airbrake: AirbrakeClient = AirbrakeClient::builder()
        .project_id_from_env().expect("Missing AIRBRAKE_PROJECT_ID")
        .project_key_from_env().expect("Missing AIRBRAKE_API_KEY")
        .environment("development")
        .build()
        .expect("Failed to build config");

    panic::set_hook(
        airbrake.panic_hook()
    );
    let _ = actix_main();
}