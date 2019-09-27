use actix_web::{error, http, HttpResponse};
use actix_web::error::BlockingError;
use failure::Fail;
use std::convert::Into;

#[derive(Fail, Debug)]
pub enum ServiceError {

    #[fail(
    display = "There was an error with the Actix async arbiter. Cause: {}",
    cause
    )]
    Actix { cause: String }
}

impl Into<AppError> for ServiceError {
    fn into(self) -> AppError {
        match self {
            ServiceError::Actix { cause } => {
                AppError::ServiceError { cause }
            }
        }
    }
}

impl Into<AuthError> for ServiceError {
    fn into(self) -> AuthError {
        match self {
            ServiceError::Actix { cause} => {
                AuthError::ServiceError { cause }
            }
        }
    }
}



#[derive(Fail, Debug)]
pub enum AuthError {

    #[fail(display = "Invalid email format!")]
    InvalidEmail,

    #[fail(display = "Team not found")]
    BadToken,

    #[fail(display = "Team already exists")]
    TeamExist,

    #[fail(display = "Service error. Reason = {}", cause)]
    ServiceError { cause: String },
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AuthError::BadToken => {
                HttpResponse::build(http::StatusCode::UNAUTHORIZED)
                    .content_type("application/json")
                    .json(r#"{"error": "Login error! Incorrect token!"}"#)
            },
            AuthError::TeamExist => {
                HttpResponse::build(http::StatusCode::FORBIDDEN)
                    .content_type("application/json")
                    .json(r#"{"error": "Team already exists!"}"#)
            },
            AuthError::InvalidEmail => {
                HttpResponse::build(http::StatusCode::BAD_REQUEST)
                    .content_type("application/json")
                    .json(r#"{"error": "Invalid email"}"#)
            },
            AuthError::ServiceError { ref cause } => {
                HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json")
                    .json(format!(r#"{{"error": "{:?}"}}"#, cause))
            }
        }
    }
}

impl From<BlockingError<AuthError>> for AuthError {

    fn from(e: BlockingError<AuthError>) -> Self {
        error!("{}", e);
        match e {
            BlockingError::Canceled => {
               ServiceError::Actix { cause: e.to_string() }.into()
            },
            BlockingError::Error(ae) => {
                ae
            }
        }
    }
}

#[derive(Fail, Debug)]
pub enum AppError {

    #[fail(display = "Game not started yet")]
    GameNotStarted,

    #[fail(display = "Context was already over")]
    GameOver,

    #[fail(display = "Service error. Reason = {}", cause)]
    ServiceError { cause: String },

}

impl error::ResponseError for AppError {

    fn error_response(&self) -> HttpResponse {
        match *self {

            AppError::GameNotStarted => {
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json(r#"{"error": "Game not started"}"#)
            },

            AppError::GameOver => {
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json(r#"{"error": "Context was already over"}"#)
            }

            AppError::ServiceError { ref cause} => {
                HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json")
                    .json(format!(r#"{{"error": "{:?}"}}"#, cause))
            }
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> Self {
        error!("{}", e);
        match e {
            BlockingError::Canceled => {
                ServiceError::Actix { cause: e.to_string() }.into()
            },
            BlockingError::Error(ae) => {
                ae
            }
        }
    }
}