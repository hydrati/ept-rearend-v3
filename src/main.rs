mod class;
mod config;
mod constant;
mod daemon;
mod hash_service;
mod response_collector;
mod scanner;
mod utils;

#[cfg(test)]
mod test;

use std::{collections::HashMap, sync::mpsc::channel, thread::spawn};

use crate::config::read_config;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use response_collector::ResponseCollector;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let cfg = crate::config::read_config().unwrap();

    let hash_map = HashMap::new();

    let (result_sender, result_receiver) = channel();
    let (cmd_sender, cmd_receiver) = channel();
    let mut daemon = daemon::Daemon::new(
        cmd_receiver,
        result_sender,
        hash_map,
        cfg.position.plugins.clone(),
    );

    let response_collector = ResponseCollector::new(result_receiver, cmd_sender, cfg.clone());

    //启动 daemon 服务
    spawn(move || {
        daemon.request();
        daemon.serve();
    });

    HttpServer::new(|| {
        App::new()
        .route("/v3/ept", web::get().to(||async {"Hello World!"}))
        // .service(ept)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
