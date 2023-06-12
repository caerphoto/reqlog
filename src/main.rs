use std::collections::HashMap;
use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::Query,
    handler::HandlerWithoutStateExt,
    http::Request,
};

use headers::HeaderMap;
use hyper::Method;


#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Ready to log requests sent to 127.0.0.1:9015 ...");

    let addr = SocketAddr::from(([127, 0, 0, 1], 9015));
    axum::Server::bind(&addr)
        .serve(handler.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    method: Method,
    headers: HeaderMap,
    Query(query): Query<HashMap<String, String>>,
    req: Request<Body>,
) -> &'static str {
    let raw_path = req.uri().to_string();
    let path = req.uri().path().to_string();
    let body = req.body();

    log::info!(
        "New request:\nMethod: {method},\nPath: {path}\nRaw path: {raw_path}\nQuery: {query:#?}\nHeaders: {headers:#?}\nBody: {body:?}"
    );

    "OK"
}
