use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    ModelError(model::Error)
}

impl From<model::Error> for Error {
    fn from(e: model::Error) -> Self {
        Self::ModelError(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED ERROR").into_response()
        }
    }
}
