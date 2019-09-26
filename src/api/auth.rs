use actix_web::{web, HttpResponse, Error, ResponseError};
use actix_identity::Identity;
use futures::{Future, future::{ok as fut_ok}};
use crate::{
    utils::{
        AuthError
    },
    database::{
        Pool,
        login_query,
        register_query,
    },
    models::{
        Login,
        Register
    }
};


pub fn register(
    data: web::Json<Register>,
    pool: web::Data<Pool>
) -> impl Future<Item = HttpResponse, Error = Error> {
    debug!("Got team_name: {}; email: {}; country: {}",
           data.team_name, data.email, data.country);

    web::block(move || register_query(data.into_inner(), pool))
        .from_err::<AuthError>()
        .then(|res | match res {
            Ok(team) => {
                Ok(HttpResponse::Ok()
                    .json(team))
            },
            Err(err) => {
                Ok(err.error_response())
            },
        })
}

pub fn login(
    data: web::Json<Login>,
    id: Identity,
    pool: web::Data<Pool>
) -> impl Future<Item = HttpResponse, Error = Error> {
    debug!("Got token: {}",
           data.token);

    web::block(move || login_query(data.into_inner(), pool))
        .from_err::<AuthError>()
        .then(move |res| match res {
            Ok(team) => {
                let team_id = team.id.to_string().to_owned();
                id.remember(team_id);
                Ok(HttpResponse::Ok()
                    .json(team))
            },
            Err(err) => {
                Ok(err.error_response())
            }
        })
}

pub fn logout(id: Identity) -> impl Future<Item = HttpResponse, Error = Error> {

    if let Some(_token) = id.identity() {
        id.forget();
    }
    fut_ok(HttpResponse::Ok()
        .json("Success"))
}
