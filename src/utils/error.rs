use actix_web::{error, http, HttpResponse};
use diesel::result::Error;
use actix_web::error::BlockingError;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum AuthError {

    #[fail(display = "Invalid email format!")]
    InvalidEmail,

    #[fail(display = "Team not found")]
    BadToken,

    #[fail(display = "Team already exists")]
    TeamExist,

    #[fail(display = "Registration error. Reason = {:?}", reason)]
    RegistrationError { reason: Error },

    #[fail(
        display = "There was an error with the Actix async arbiter. Cause: {}",
        cause
    )]
    Actix { cause: String }

}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AuthError::BadToken => {
                HttpResponse::build(http::StatusCode::UNAUTHORIZED)
                    .content_type("application/json")
                    .json("\"error\": \"Login error! Incorrect token!\"")
            },
            AuthError::TeamExist => {
                HttpResponse::build(http::StatusCode::FORBIDDEN)
                    .content_type("application/json")
                    .json("\"error\": \"Team already exists!\"")
            },
            AuthError::InvalidEmail => {
                HttpResponse::build(http::StatusCode::BAD_REQUEST)
                    .content_type("application/json")
                    .json("\"error\": \"Invalid email\"")
            },
            AuthError::RegistrationError {ref reason} => {
                HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json")
                    .json(format!("\"RegistrationError\": \"{:?}\"", reason))
            },
            AuthError::Actix { ref cause } => {
                HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json")
                    .json(format!("\"ActixError\": \"{:?}\"", cause))
            }
        }
    }
}

impl From<BlockingError<AuthError>> for AuthError {

    fn from(e: BlockingError<AuthError>) -> Self {
        error!("{}", e);
        match e {
            BlockingError::Canceled => {
               AuthError::Actix { cause: e.to_string() }
            },
            BlockingError::Error(ae) => {
                ae
            }
        }
    }
}