use redis::RedisError;
use salvo::{prelude::*, Writer};
use std::time::{SystemTimeError};
use sea_orm::sqlx;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomError {
    NotFound(String),
    TeraError(String),
    ParseError(String),
    InternalServerError(String),
}

#[async_trait]
impl Writer for CustomError {
    async fn write(
        self,
        _req: &mut salvo::Request,
        depot: &mut salvo::Depot,
        res: &mut salvo::Response,
    ) {
        let status = match self {
            CustomError::NotFound(_) => StatusCode::NOT_FOUND,
            CustomError::TeraError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::ParseError(_) => StatusCode::BAD_REQUEST,
            CustomError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error_text = match self.clone() {
            CustomError::NotFound(m) => m,
            CustomError::TeraError(m) => m,
            CustomError::ParseError(m) => m,
            CustomError::InternalServerError(m) => m,
        };

        res.status_code(status);
        res.render(Json(&error_text));
    }
}

impl  From<salvo::http::ParseError> for CustomError {
    fn from(e: salvo::http::ParseError) -> Self {
        CustomError::ParseError(e.to_string())
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

impl From<RedisError> for CustomError {
    fn from(e: RedisError) -> Self {
        CustomError::InternalServerError(e.to_string())
    }
}

impl From<serde_json::Error>  for CustomError {
    fn from(e: serde_json::Error) -> Self {
        CustomError::ParseError(e.to_string())
    }
}

impl From<SystemTimeError> for CustomError {
    fn from(e: SystemTimeError) -> Self {
        CustomError::InternalServerError(e.to_string())
    }
}