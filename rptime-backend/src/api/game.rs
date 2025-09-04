use crate::{
    database::game::{get_scoreboard, get_task, get_team, load_map, push_flag},
    models::{GetFlag},
    utils::{map_getter, AppData},
};
use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpResponse, Responder, ResponseError};
use std::path::PathBuf;

#[get("/")]
pub async fn index(team_id: Identity, app: web::Data<AppData>) -> impl Responder {
    let id = team_id.id();
    let id = id.parse::<i32>().unwrap();

    let tasks = web::block(move || load_map(id, &app.pool))
        .await
        .map_err(|err| err.error_response());
    Ok(HttpResponse::Ok().json(tasks))
}

#[get("/scoreboard")]
pub async fn scoreboard(app: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let teams_scores = web::block(move || get_scoreboard(&app.pool))
        .await
        .map_err(|err| err.error_response())?;
    Ok(HttpResponse::Ok().json(teams_scores))
}

#[get("/me")]
pub async fn me(
    team_id: Identity,
    app: web::Data<AppData>,
) -> Result<HttpResponse, Error> {

    todo!("Get current TeamInfo")

    // let id = team_id.identity().unwrap();
    // let id = id.parse::<i32>().unwrap();
    //
    // let team = web::block(move || get_team(id, &app.pool))
    //     .await
    //     .map_err(|err| err.error_response())?;
    // Ok(HttpResponse::Ok().json(team))
}

#[get("/map-config")]
pub async fn map_config(app: web::Data<PathBuf>) -> Result<HttpResponse, Error> {
    todo!("Return map info")
    // let map = web::block(move || map_getter(app.get_ref()))
    //     .await
    //     .map_err(|err| err.error_response())?;
    // Ok(HttpResponse::Ok().json(map))
}
