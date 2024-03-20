use std::error::Error;
use std::fmt::Debug;

use actix_web::{error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(Error)]
pub enum ApiError {
    #[error("Internal Server Error occurred")]
    InternalError,

    #[error("Bad Request")]
    BadClientData,

    #[error("DB Error: {0}")]
    DBError(#[from] diesel::result::Error),

}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        use diesel::result::*;
        match *self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadClientData => StatusCode::BAD_REQUEST,
            ApiError::DBError(Error::NotFound) => StatusCode::NOT_FOUND,
            ApiError::DBError(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => StatusCode::CONFLICT,
            ApiError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.to_string())
    }
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}