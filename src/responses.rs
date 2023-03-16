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

pub fn bad_request(user_error: UserErrorMessage) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(user_error.message()))
        .unwrap()
}

pub enum UserErrorMessage {
    QueryString,
    InvalidUserId,
}

impl UserErrorMessage {
    fn message(&self) -> String {
        let mut error_string = String::from("error: ");
        match self {
            UserErrorMessage::QueryString => {
                error_string.push_str("invalid query string parameter | expected 'id='");
            }
            UserErrorMessage::InvalidUserId => {
                error_string.push_str("user id not found");
            }
        }
        error_string
    }
}

