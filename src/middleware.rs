use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

// use actix_service::{Service, Transform};
use actix_identity::RequestIdentity;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Error, HttpResponse, ResponseError};

use futures::future::{ok, Ready};
use futures::Future;

use crate::database::check_game;
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

// pub struct CheckGame;

// impl<S, B> Transform<S, ServiceRequest> for CheckGame
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
// {
//     type Response = S::Response;
//     type Error = Error;
//     type Transform = CheckGameMiddleware<S>;
//     type InitError = ();
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(CheckGameMiddleware {
//             service: Rc::new(service),
//         })
//     }
// }

// pub struct CheckGameMiddleware<S> {
//     service: Rc<S>,
// }

// impl<S, B> Service<ServiceRequest> for CheckGameMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         let data = req.app_data::<AppData>().clone().unwrap();

//         let now = chrono::Local::now().naive_local();

//         let mut start_game = *data.start_game.lock().unwrap();
//         let mut end_game = *data.end_game.lock().unwrap();

//         let res = if now < start_game {
//             Err(AppError::GameNotStarted)
//         } else if now > end_game {
//             web::block(move || check_game(&data.pool))
//                 .from_err::<AppError>()
//                 .then(|res| match res {
//                     Ok((start, end)) => {
//                         start_game = start;
//                         end_game = end;
//                         Ok(())
//                     }
//                     Err(err) => Err(err),
//                 })
//                 .wait()
//         } else {
//             Ok(())
//         };

//         match res {
//             Ok(_res) => {
//                 if req.path() == "/api/v1/register" {
//                     Either::B(ok(req.into_response(
//                         HttpResponse::BadRequest()
//                             .content_type("application/json")
//                             .json(ResponseJsonError {
//                                 error: "Contest already running. You can't register".to_string(),
//                             })
//                             .into_body(),
//                     )))
//                 } else {
//                     Either::A(self.service.call(req))
//                 }
//             }
//             Err(err) => {
//                 if req.path() == "/api/v1/register" {
//                     match err {
//                         AppError::GameNotStarted => Either::A(self.service.call(req)),
//                         _ => Either::B(ok(req.into_response(err.error_response().into_body()))),
//                     }
//                 } else {
//                     Either::B(ok(req.into_response(err.error_response().into_body())))
//                 }
//             }
//         }
//     }
// }
