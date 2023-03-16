#![allow(warnings)]
mod process;
mod responses;

#[macro_use]
extern crate log;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use responses::UserErrorMessage;
use std::convert::Infallible;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();
    let make_svc = make_service_fn(move |_conn| async { Ok::<_, Infallible>(service_fn(handler)) });
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    info!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn handle_get(query: Option<&str>) -> Response<Body> {
    match &query {
        Some(value) => {
            if !value.starts_with("id=") {
                responses::bad_request(UserErrorMessage::QueryString)
            } else {
                let id: u32 = value.replace("id=", "").parse().unwrap_or_default();
                let result_message = process::process_request(id);
                Response::new(Body::from(result_message))
            }
        }
        None => responses::bad_request(UserErrorMessage::QueryString),
    }
}

async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.uri().path() {
        ("/api/v1/calculate") => match req.method() {
            &Method::GET => Ok(handle_get(req.uri().query())),
            _ => Ok(responses::method_not_allowed()),
        },
        _ => Ok(responses::not_found()),
    }
}
