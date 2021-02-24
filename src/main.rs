#![allow(unused_imports, unused_variables)]
pub use kube_runtime::controller::*;
use prometheus::{Encoder, TextEncoder};
use std::cell::Cell;
use tracing::{debug, error, info, trace, warn};

struct SchedulerData {
    requests: Cell<usize>,
}

use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

#[get("/metrics")]
async fn metrics(data: web::Data<SchedulerData>, _req: HttpRequest) -> impl Responder {
    data.requests.set(data.requests.get() + 1);
    HttpResponse::Ok().body(data.requests.get().to_string())
}

#[get("/health")]
async fn health(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    // let state = c.state().await;
    HttpResponse::Ok().json("ok")
}

#[actix_rt::main]
async fn main() -> Result<(), ()> {
// fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let client = kube::Client::try_default().await.expect("create client");
    // let (manager, drainer) = Manager::new(client).await;

    let server = HttpServer::new(move || {
        App::new()
            .data(SchedulerData{ requests: Cell::new(0) })
            .wrap(middleware::Logger::default().exclude("/health"))
            .service(index)
            .service(health)
            .service(metrics)
    })
    .bind("0.0.0.0:8080")
    .expect("Can not bind to 0.0.0.0:8080")
    .shutdown_timeout(0);

    tokio::select! {
        // _ = drainer => warn!("controller drained"),
        _ = server.run() => info!("actix exited"),
    }
    Ok(())
}