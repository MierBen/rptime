use actix_identity::RequestIdentity;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;

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
                        .json("\"error\": \"You didn't login!\"")
                        .into_body()
                )))
            }
        }
    }
}
