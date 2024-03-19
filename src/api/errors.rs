use actix_web::{error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use derive_more::Display;
use diesel::result::DatabaseErrorKind;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Internal Error")]
    InternalError,

    #[display(fmt = "Bad Request")]
    BadClientData,

    #[display(fmt = "Resource not found")]
    NotFound,

    #[display(fmt = "Unique constraint violated")]
    UniqueViolation,
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadClientData => StatusCode::BAD_REQUEST,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::UniqueViolation => StatusCode::CONFLICT,
        }
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(value: diesel::result::Error) -> Self {
        use diesel::result::{DatabaseErrorKind, Error};
        match value {
            Error::NotFound => ApiError::NotFound,
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => ApiError::UniqueViolation,
            _ => ApiError::InternalError
        }
    }
}