use hyper::{Body, Response, StatusCode};

pub fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}

pub fn method_not_allowed() -> Response<Body> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .unwrap()
}

pub fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("an unexpected error occurred"))
        .unwrap()
}

pub fn bad_request(user_error: WebErrorMessage) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(user_error.message()))
        .unwrap()
}

pub fn accepted() -> Response<Body> {
    Response::builder()
        .status(StatusCode::ACCEPTED)
        .body(Body::empty())
        .unwrap()
}

pub enum WebErrorMessage {
    ZeroUserId,
}

impl WebErrorMessage {
    fn message(&self) -> String {
        let mut error_string = String::from("error: ");
        match self {
            WebErrorMessage::ZeroUserId => {
                error_string.push_str("user id cannot be 0");
            }
        }
        error_string
    }
}
