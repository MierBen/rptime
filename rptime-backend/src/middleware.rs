use std::pin::Pin;
use std::task::{Context, Poll};


use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::Future;

use crate::utils::{AppData, ResponseJsonError};

pub struct CheckAuthService;

impl<S, B> Transform<S, ServiceRequest> for CheckAuthService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
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

impl<S, B> Service<ServiceRequest> for CheckAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    // fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    //     self.service.poll_ready(cx)
    // }

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let is_logged = false;// req.get_session().;

        if !is_logged && req.path() != "/api/v1/login" {
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
        Box::pin(async move {
            let res = srv.await?;
            Ok(res)
        })
    }
}

pub struct CheckGame;

impl<S, B> Transform<S, ServiceRequest> for CheckGame
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
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

impl<S, B> Service<ServiceRequest> for CheckGameMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let data = req.app_data::<AppData>().clone().unwrap();
        let path = req.path();
        let now = chrono::Local::now().naive_local();
        let start_game = data.start_game;
        let end_game = data.end_game;

        if path == "/api/v1/register" {
            if now < start_game || now >= end_game {
                let error = if now < start_game {
                    "Contest already running. You can't register".to_string()
                } else {
                    "Contest already finished.".to_string()
                };
                return Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::BadRequest()
                            .content_type("application/json")
                            .json(ResponseJsonError { error })
                            .into_body()
                    ))
                });
            }
        }

        let srv = self.service.call(req);

        Box::pin(async move {
            let res = srv.await?;
            Ok(res)
        })
    }
}
