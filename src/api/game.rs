use actix_web::{web, HttpResponse, Error, ResponseError};
use actix_identity::Identity;
use futures::{Future, future::{ok as fut_ok}};
use crate::{
    utils::{
        AuthError
    },
    database::{
        Pool,
    },
//    models::{
//        Login,
//        Register
//    }
};


pub fn index(
    id: Identity,
    pool: web::Data<Pool>
) -> impl Future<Item = HttpResponse, Error = Error> {
    if let Some(id) = id.identity() {
        fut_ok(HttpResponse::Ok()
            .json(format!("Hello, team number {}", id)))
    } else {
        fut_ok(HttpResponse::Ok()
            .json("OOoops"))
    }
}