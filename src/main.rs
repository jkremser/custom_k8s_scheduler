#![allow(unused_imports, unused_variables)]
use k8s_openapi::api::core::v1::Pod;
use prometheus::{Encoder, TextEncoder};
use std::cell::Cell;
use tracing::{debug, error, info, trace, warn};

use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, Resource, WatchEvent},
    Client,
};

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
    let client: Client = Client::try_default().await.expect("Unable to initialize the k8s client, is Kubernetes up and running?");
    let pods: Api<Pod> = Api::namespaced(client, "kube-system");
    let lp = ListParams::default();
    for p in pods.list(&lp).await {
        info!("Found Pod: {:?}", &p);
        // info!("Found Pod: {}", Resource::name(&p));
    }
    HttpResponse::Ok().json("ok")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");


    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

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