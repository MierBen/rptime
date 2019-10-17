use actix_web::error::BlockingError;
use actix_web::{error, HttpResponse};
use failure::Fail;
use serde::{Deserialize, Serialize};
use std::convert::Into;

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(
        display = "There was an error with the Actix async arbiter. Cause: {}",
        cause
    )]
    Actix { cause: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseJsonError {
    pub error: String,
}

impl Into<AppError> for ServiceError {
    fn into(self) -> AppError {
        match self {
            ServiceError::Actix { cause } => AppError::ServiceError { cause },
        }
    }
}

impl Into<AuthError> for ServiceError {
    fn into(self) -> AuthError {
        match self {
            ServiceError::Actix { cause } => AuthError::ServiceError { cause },
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

    #[fail(display = "Field {} is empty", field)]
    FieldEmpty { field: String },
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AuthError::BadToken => HttpResponse::Unauthorized()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: "Login error! Incorrect token!".to_string(),
                }),
            AuthError::TeamExist => HttpResponse::Forbidden()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: "Team already exists!".to_string(),
                }),
            AuthError::InvalidEmail => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: "Invalid email".to_string(),
                }),
            AuthError::ServiceError { ref cause } => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: cause.to_string(),
                }),
            AuthError::FieldEmpty { ref field } => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: format!("Field {} is empty", field),
                }),
        }
    }
}

impl From<BlockingError<AuthError>> for AuthError {
    fn from(e: BlockingError<AuthError>) -> Self {
        error!("{}", e);
        match e {
            BlockingError::Canceled => ServiceError::Actix {
                cause: e.to_string(),
            }
            .into(),
            BlockingError::Error(ae) => ae,
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

    #[fail(display = "Task not found!")]
    TaskNotFound,

    #[fail(display = "Task already solved!")]
    TaskAlreadySolved,

    #[fail(display = "Task doesn't open yet!")]
    TaskNotOpenned,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::GameNotStarted => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: "Game not started".to_string(),
                }),
            AppError::GameOver => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: "Context was already over".to_string(),
                }),
            AppError::ServiceError { ref cause } => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: cause.to_string(),
                }),
            AppError::TaskNotFound => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: "Task not found".to_string(),
                }),
            AppError::TaskAlreadySolved => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: "Task already solved!".to_string(),
                }),
            AppError::TaskNotOpenned => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(ResponseJsonError {
                    error: "Task doesn't open yet!".to_string(),
                }),
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> Self {
        error!("{}", e);
        match e {
            BlockingError::Canceled => ServiceError::Actix {
                cause: e.to_string(),
            }
            .into(),
            BlockingError::Error(ae) => ae,
        }
    }
}
