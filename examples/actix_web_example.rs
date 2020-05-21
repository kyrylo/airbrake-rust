#[macro_use]
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
    let airbarke_project_id = "your-project-id".to_string();
    let airbrake_project_key = "your-project-key".to_string();
    let config = AirbrakeConfig::builder()
        .project_id(airbarke_project_id)
        .project_key(airbrake_project_key)
        .build()
        .expect("Failed to build config");
    let airbrake_client = AirbrakeClient::new(config);

    // Configure the hook
    panic::set_hook(Box::new(move |panic_info| {
        let panic_backtrace = backtrace::Backtrace::new();
        let notice_error = NoticeError::builder("foo")
            .raw_backtrace(panic_backtrace)
            .build();
        airbrake_client.new_notice_builder()
            .add_notice(notice_error)
            .build()
            .send();
    }));

    actix_main();
}