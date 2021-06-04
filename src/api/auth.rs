use crate::{
    database::{login_query, register_query},
    models::{Login, Register},
    utils::{AppData, AuthError},
};
use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse, Responder, ResponseError};

pub async fn register(
    data: web::Json<Register>,
    pool: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    debug!(
        "Got team_name: {}; email: {}; country: {}",
        data.team_name, data.email, data.country
    );

    let team = web::block(move || register_query(data.into_inner(), &pool.into_inner().pool))
        .await
        .map_err(|err| err.error_response())?;
    Ok(HttpResponse::Ok().json(team))
}

pub async fn login(
    data: web::Json<Login>,
    id: Identity,
    app: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    debug!("Got token: {}", data.token);

    let team = web::block(move || login_query(data.into_inner(), &app.pool))
        .await
        .map_err(|err| err.error_response())?;
    let team_id = team.id.to_string().to_owned();
    id.remember(team_id);
    Ok(HttpResponse::Ok().json(team))
}

pub async fn logout(id: Identity) -> impl Responder {
    if let Some(_token) = id.identity() {
        id.forget();
    }
    HttpResponse::Ok().finish()
}
