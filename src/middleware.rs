use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

// use actix_service::{Service, Transform};
use actix_identity::RequestIdentity;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse, ResponseError};

use futures::future::{ok, Ready};
use futures::Future;

use crate::utils::{AppData, AppError, ResponseJsonError};

pub struct CheckAuthService;

impl<S, B> Transform<S> for CheckAuthService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = Error;
    type Transform = CheckAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckAuthMiddleware { service })
    }
}

pub struct CheckAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Request = S::Request;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let identity = RequestIdentity::get_identity(&req).unwrap_or("".into());
        let is_logged = !identity.is_empty();
        let unauthorized = !is_logged && req.path() != "/api/v1/login";

        if unauthorized {
            return Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseJsonError {
                            error: "You didn't login!".to_string(),
                        })
                        .into_body(),
                ))
            });
        }

        let srv = self.service.call(req);
        Box::pin(async move { srv.await })
    }
}

pub struct CheckGame;

impl<S, B> Transform<S> for CheckGame
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = Error;
    type Transform = CheckGameMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckGameMiddleware { service })
    }
}

pub struct CheckGameMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckGameMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let data = req.app_data::<AppData>().clone().unwrap();

        let now = chrono::Local::now().naive_local();

        let start_game = data.start_game;
        let end_game = data.end_game;
        let srv = self.service.call(req);

        if now < start_game && req.path() == "/api/v1/register" {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .json(ResponseJsonError {
                            error: "Contest already running. You can't register".to_string(),
                        })
                        .into_body(),
                ))
            })
        } else if now > end_game && req.path() == "/api/v1/register" {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .json(ResponseJsonError {
                            error: "Contest already finished.".to_string(),
                        })
                        .into_body(),
                ))
            })
        } else {
            Box::pin(async move { srv.await })
        }
    }
}
