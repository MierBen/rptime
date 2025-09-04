use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpResponse};
use crate::database::game::get_task;

use crate::utils::AppData;
use crate::models::GetFlag;

#[get("/task/{task_id}")]
pub async fn task(
    task_id: web::Path<i32>,
    team_id: Identity,
    app: web::Data<AppData>,
) -> Result<HttpResponse, Error> {

    todo!("Get task by id")

    // let id = team_id.identity().unwrap();
    // let id = id.parse::<i32>().unwrap();
    //
    // let task = web::block(move || get_task(task_id.into_inner(), id, &app.pool))
    //     .await
    //     .map_err(|err| err.error_response())?;
    // Ok(HttpResponse::Ok().json(task))
}

#[post("/task/{task_id}/solve")]
pub async fn solve(
    path: web::Path<i32>,
    data: web::Json<GetFlag>,
    team_id: Identity,
    app: web::Data<AppData>,
) -> Result<HttpResponse, Error> {

    todo!("Trying to solve task")

    // let id = team_id.identity().unwrap();
    // let id = id.parse::<i32>().unwrap();
    //
    // let solved = web::block(move || push_flag(id, path.into_inner(), &data.flag, &app.pool))
    //     .await
    //     .map_err(|err| err.error_response())?;
    // Ok(HttpResponse::Ok().json(solved))
}