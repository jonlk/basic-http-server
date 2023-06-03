#![allow(warnings)]
mod process;
mod responses;

#[macro_use]
extern crate log;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use process::process_request;
use regex::Regex;

use std::convert::Infallible;

use crate::responses::*;

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
        Ok(value) => api_response(StatusCode::ACCEPTED, None), //value not used here but could be
        Err(err) => api_response(StatusCode::BAD_REQUEST, Some(&err.to_string())),
    }
}
async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let req_string = req.uri().path_and_query().unwrap().as_str();
    let re = Regex::new(r"^/api/v1/calculate\?id=(\d+$)").unwrap();
    let mut status_code: StatusCode;

    if !re.is_match(&req_string) {
        status_code = StatusCode::NOT_FOUND;
        info!("{:?}, {:?}", &req_string, &status_code);
        Ok(api_response(status_code, None))
    } else {
        match req.method() {
            &Method::GET => {
                let capture = re.captures(&req_string).unwrap();
                let id: u32 = String::from(&capture[1]).parse().unwrap_or_default();
                if id == 0 {
                    status_code = StatusCode::NOT_FOUND;
                    Ok(api_response(status_code, Some("Path not found")))
                } else {
                    Ok(handle_get(id))
                }
            }
            _ => Ok({
                status_code = StatusCode::METHOD_NOT_ALLOWED;
                api_response(status_code, None)
            }),
        }
    }
}
