use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug,Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
    Unauthored(String),
}

#[derive(Debug,Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}


impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                log::error!("Database error occurred: {msg}");
                "Database error".into()
            },
            MyError::ActixError(msg) => {
                log::error!("Server error occurred: {msg}");
                "Internal server error".into()
            },
            MyError::NotFound(msg) => {
                log::error!("Not found error occurred: {msg}");
                msg.into()
            },
            MyError::InvalidInput(msg) => {
                log::error!("invalid parameters received: {msg}");
                msg.into()
            },
            MyError::Unauthored(msg) => {
                log::error!("unauthor error occurred: {msg}");
                "authorition error".into()
            },
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_) | MyError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            MyError::Unauthored(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{self}")
    }
}

impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<error::Error> for MyError {
    fn from(err: error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}