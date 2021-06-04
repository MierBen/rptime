use crate::{
    database::{get_scoreboard, get_task, get_team, load_map, push_flag},
    models::{GetFlag, TaskShortInfo},
    utils::{map_getter, AppData, AppError, ResponseJsonError},
};
use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse, Responder, ResponseError};
use futures::Future;
use std::path::PathBuf;

pub async fn index(team_id: Identity, app: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let id = team_id.identity().unwrap();
    let id = id.parse::<i32>().unwrap();

    let tasks: Vec<TaskShortInfo> = web::block(move || load_map(id, &app.pool))
        .await
        .map_err(|err| err.error_response())?;
    Ok(HttpResponse::Ok().json(tasks))
}

pub async fn task(
    task_id: web::Path<i32>,
    team_id: Identity,
    app: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    let id = team_id.identity().unwrap();
    let id = id.parse::<i32>().unwrap();

    let task = web::block(move || get_task(task_id.into_inner(), id, &app.pool))
        .await
        .map_err(|err| err.error_response())?;
    Ok(HttpResponse::Ok().json(task))
}

pub async fn scoreboard(app: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let teams_scores = web::block(move || get_scoreboard(&app.pool))
        .await
        .map_err(|err| err.error_response())?;
    Ok(HttpResponse::Ok().json(teams_scores))
}

pub async fn solve(
    path: web::Path<i32>,
    data: web::Json<GetFlag>,
    team_id: Identity,
    app: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    let id = team_id.identity().unwrap();
    let id = id.parse::<i32>().unwrap();

    let solved = web::block(move || push_flag(id, path.into_inner(), &data.flag, &app.pool))
        .await
        .map_err(|err| err.error_response())?;
    Ok(HttpResponse::Ok().json(solved))
}

pub async fn me(team_id: Identity, app: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let id = team_id.identity().unwrap();
    let id = id.parse::<i32>().unwrap();

    let team = web::block(move || get_team(id, &app.pool))
        .await
        .map_err(|err| err.error_response())?;
    Ok(HttpResponse::Ok().json(team))
}

pub async fn map_config(app: web::Data<PathBuf>) -> Result<HttpResponse, Error> {
    let map = web::block(move || map_getter(app.get_ref()))
        .await
        .map_err(|err| err.error_response())?;
    Ok(HttpResponse::Ok().json(map))
}
