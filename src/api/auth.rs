use crate::{
    database::{login_query, register_query},
    models::{Login, Register},
    utils::{AppData, AuthError},
};
use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::{future::ok as fut_ok, Future};

pub fn register(
    data: web::Json<Register>,
    app: web::Data<AppData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    debug!(
        "Got team_name: {}; email: {}; country: {}",
        data.team_name, data.email, data.country
    );

    web::block(move || register_query(data.into_inner(), &app.pool))
        .from_err::<AuthError>()
        .then(|res| match res {
            Ok(team) => Ok(HttpResponse::Ok().json(team)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn login(
    data: web::Json<Login>,
    id: Identity,
    app: web::Data<AppData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    debug!("Got token: {}", data.token);

    web::block(move || login_query(data.into_inner(), &app.pool))
        .from_err::<AuthError>()
        .then(move |res| match res {
            Ok(team) => {
                let team_id = team.id.to_string().to_owned();
                id.remember(team_id);
                Ok(HttpResponse::Ok().json(team))
            }
            Err(err) => Ok(err.error_response()),
        })
}

pub fn logout(id: Identity) -> impl Future<Item = HttpResponse, Error = Error> {
    if let Some(_token) = id.identity() {
        id.forget();
    }
    fut_ok(HttpResponse::Ok().finish())
}
