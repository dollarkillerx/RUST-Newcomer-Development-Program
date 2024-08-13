use std::fmt;
use ntex::{
    http::StatusCode,
    web::{WebResponseError, HttpResponse},
};
use ntex::web::HttpRequest;

#[derive(Debug,Clone)]
pub enum CustomError {
    InternalServerError(String),
    NotFound(String),
}

impl WebResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        // match self {
        //     CustomError::InternalServerError(e) => HttpResponse::build(self.status_code()).body(e),
        //     CustomError::NotFound(e) => HttpResponse::build(self.status_code()).body(e),
        // }

        HttpResponse::new(self.status_code()).set_body(
            match self {
                CustomError::InternalServerError(e) => e,
                CustomError::NotFound(e) => e,
            }.into()
        )
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::InternalServerError(e) => write!(f, "{}", e),
            CustomError::NotFound(e) => write!(f, "{}", e),
        }
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => CustomError::NotFound("not found".into()),
            _ => CustomError::InternalServerError(e.to_string()),
        }
    }
}