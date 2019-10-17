use crate::{
    database::{get_scoreboard, get_task, get_team, load_map, push_flag},
    models::GetFlag,
    utils::{map_getter, AppData, AppError, ResponseJsonError},
};
use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::Future;
use std::path::PathBuf;

pub fn index(
    team_id: Identity,
    app: web::Data<AppData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let id = team_id.identity().unwrap();
    let id = id.parse::<i32>().unwrap();

    web::block(move || load_map(id, &app.pool))
        .from_err::<AppError>()
        .then(|res| match res {
            Ok(tasks) => Ok(HttpResponse::Ok().json(tasks)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn task(
    task_id: web::Path<i32>,
    team_id: Identity,
    app: web::Data<AppData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let id = team_id.identity().unwrap();
    let id = id.parse::<i32>().unwrap();

    web::block(move || get_task(task_id.into_inner(), id, &app.pool))
        .from_err::<AppError>()
        .then(|res| match res {
            Ok(task) => Ok(HttpResponse::Ok().json(task)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn scoreboard(app: web::Data<AppData>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || get_scoreboard(&app.pool))
        .from_err::<AppError>()
        .then(|res| match res {
            Ok(teams_scores) => Ok(HttpResponse::Ok().json(teams_scores)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn solve(
    path: web::Path<i32>,
    data: web::Json<GetFlag>,
    team_id: Identity,
    app: web::Data<AppData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let id = team_id.identity().unwrap();
    let id = id.parse::<i32>().unwrap();

    web::block(move || push_flag(id, path.into_inner(), &data.flag, &app.pool))
        .from_err::<AppError>()
        .then(|res| match res {
            Ok(solved) => Ok(HttpResponse::Ok().json(solved)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn me(
    team_id: Identity,
    app: web::Data<AppData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let id = team_id.identity().unwrap();
    let id = id.parse::<i32>().unwrap();

    web::block(move || get_team(id, &app.pool))
        .from_err::<AppError>()
        .then(|res| match res {
            Ok(team) => Ok(HttpResponse::Ok().json(team)),
            Err(err) => Ok(err.error_response()),
        })
}

pub fn map_config(app: web::Data<PathBuf>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || map_getter(app.get_ref())).then(|res| match res {
        Ok(map) => Ok(HttpResponse::Ok().json(map)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(ResponseJsonError {
            error: "Can't update game map config".to_string(),
        })),
    })
}
