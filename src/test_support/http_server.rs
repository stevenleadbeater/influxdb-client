use actix_web::{Responder, HttpResponse, post, App};
use actix_cors::Cors;
use actix_test::TestServer;
use log::{info};

#[post("/success")]
pub async fn post() -> impl Responder {
    info!("POST /");
    HttpResponse::Ok().body("test")
}

#[post("/")]
pub async fn post_error() -> impl Responder {
    info!("POST /");
    HttpResponse::InternalServerError()
}

#[post("/body-response")]
pub async fn post_error_body() -> impl Responder {
    info!("GET /");
    HttpResponse::InternalServerError().body("Some terrible error")
}

#[post("/fails/api/v2/query")]
pub async fn fake_influxdb_fails() -> impl Responder {
    info!("POST /");
    HttpResponse::InternalServerError()
}

#[post("/fails-body-response/api/v2/query")]
pub async fn fake_influxdb_fails_with_body() -> impl Responder {
    info!("POST /");
    HttpResponse::InternalServerError().body("Some terrible error")
}

#[post("/success/api/v2/query")]
pub async fn fake_influxdb_success() -> impl Responder {
    info!("POST /");
    HttpResponse::Ok().body("test")
}

#[post("/fails/api/v2/write")]
pub async fn fake_write_influxdb_fails() -> impl Responder {
    info!("POST /");
    HttpResponse::InternalServerError()
}

#[post("/fails-body-response/api/v2/write")]
pub async fn fake_write_influxdb_fails_with_body() -> impl Responder {
    info!("POST /");
    HttpResponse::InternalServerError().body("Some terrible error")
}

#[post("/success/api/v2/write")]
pub async fn fake_write_influxdb_success() -> impl Responder {
    info!("POST /");
    HttpResponse::Ok().body("test")
}

#[allow(dead_code)]
pub fn setup_test_harness() -> TestServer {
    actix_test::start(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .service(post)
            .service(post_error)
            .service(post_error_body)
            .service(fake_influxdb_fails)
            .service(fake_influxdb_fails_with_body)
            .service(fake_influxdb_success)
            .service(fake_write_influxdb_fails)
            .service(fake_write_influxdb_fails_with_body)
            .service(fake_write_influxdb_success)
    })
}
