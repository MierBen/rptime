use actix_identity::RequestIdentity;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse, web, ResponseError};
use futures::future::{ok, Either, FutureResult, Future};
use futures::Poll;

use crate::database::check_game;
use crate::utils::AppError;

pub struct CheckLogin;

impl <S, B> Transform<S> for CheckLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S
}


impl<S, B> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if let Some(_) = req.get_identity() {
            Either::A(self.service.call(req))
        } else {
            if req.path() == "/api/v1/login" {
                Either::A(self.service.call(req))
            } else {
                Either::B(ok(req.into_response(
                    HttpResponse::BadRequest()
                        .json("error: You didn't login!")
                        .into_body()
                )))
            }
        }
    }
}

pub struct CheckGame;

impl <S, B> Transform<S> for CheckGame
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckGameMiddleware<S>;
    type InitError = ();
    type Future = FutureResult<Self::Transform, Self::InitError>;

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
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {

        let data = req.app_data().clone().unwrap();

        let res = web::block( move || check_game(data))
            .from_err::<AppError>()
            .wait();

        match res {
             Ok(_res) => {
                 if req.path() == "/api/v1/register" {
                     Either::B(ok(req.into_response(
                        HttpResponse::BadRequest()
                            .content_type("application/json")
                            .json(r#"{"error": "Contest already running. You can't register"}"#)
                            .into_body()
                     )))
                 } else {
                     Either::A(self.service.call(req))
                 }

             },
             Err(err) => {
                 if req.path() == "/api/v1/register" {
                     match err {
                         AppError::GameNotStarted => Either::A(self.service.call(req)),
                         _ =>   Either::B(
                             ok(req.into_response(
                                 err
                                     .error_response()
                                     .into_body())
                             ))
                     }

                 } else {
                     Either::B(
                         ok(req.into_response(
                             err
                                 .error_response()
                                 .into_body())
                         ))
                 }
             }
         }
    }
}
