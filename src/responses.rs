use hyper::{Body, Response, StatusCode};

pub fn api_response(status_code: StatusCode, message: Option<&str>) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(match message {
            Some(m) => Body::from(String::from(m)),
            None => Body::empty(),
        })
        .unwrap()
}
