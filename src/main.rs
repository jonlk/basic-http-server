#![allow(warnings)]
mod process;
mod responses;

#[macro_use]
extern crate log;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use process::process_request;
use regex::Regex;
use responses::{accepted, bad_request, method_not_allowed, not_found};

use std::convert::Infallible;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    let make_svc = make_service_fn(move |_conn| async { Ok::<_, Infallible>(service_fn(handler)) });

    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn handle_get(id: u32) -> Response<Body> {
    let pr = process_request(id);
    match pr {
        Ok(value) => accepted(),
        Err(err) => Response::new(Body::from(err.to_string())),
    }
}
async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let req_string = req.uri().path_and_query().unwrap().as_str();
    let re = Regex::new(r"^/api/v1/calculate\?id=(\d+$)").unwrap();
    
    info!("Request received");

    if !re.is_match(&req_string) {
        Ok(not_found())
    } else {
        match req.method() {
            &Method::GET => {
                let capture = re.captures(&req_string).unwrap();
                let id: u32 = String::from(&capture[1]).parse().unwrap_or_default();
                if id == 0 {
                    Ok(not_found())
                } else {
                    Ok(handle_get(id))
                }
            }
            _ => Ok(method_not_allowed()),
        }
    }
}
