use actix_web::http::StatusCode;
use actix_web::{error, http::header::ContentType, HttpResponse};
use derive_more::{Display, Error};
use std::fmt::Debug;

pub mod database;

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "not found")]
    NotFound,

    #[display(fmt = "bad request")]
    MalformedBody,

    #[display(fmt = "internal error")]
    InternalServerError,
}

impl error::ResponseError for CustomError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        // TODO: Complete
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MalformedBody => StatusCode::BAD_REQUEST,
        }
    }
}
