use hyper::{Body, Response, StatusCode};

pub fn status(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}

pub fn status_with_message(status_code: StatusCode, message: &str) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::from(String::from(message)))
        .unwrap()
}
