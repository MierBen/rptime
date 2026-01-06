use crate::{
    models::{PushFlag, Scoreboard, Task, TaskFullInfo, TaskShortInfo, TeamGameInfo},
    utils::AppError,
};
use chrono::NaiveDateTime;
use diesel::{prelude::*, RunQueryDsl};
use regex::Regex;

use super::Pool;

fn check_access(task_id: i32, team_id: i32, pool: &Pool) -> Result<(), AppError> {
    use crate::models::schema::{tasks, team_game};

    let conn = &pool.get().unwrap();

    let task_keys: Vec<i32> = tasks::table
        .select(tasks::keys_condition)
        .filter(tasks::id.eq(&task_id))
        .first(conn)
        .map_err(|err| {
            if err == diesel::NotFound {
                AppError::TaskNotFound
            } else {
                AppError::ServiceError {
                    cause: err.to_string(),
                }
            }
        })?;

    let team_keys: Vec<i32> = team_game::table
        .select(team_game::keys_owned)
        .filter(team_game::team_id.eq(&team_id))
        .first(conn)
        .map_err(|err| AppError::ServiceError {
            cause: err.to_string(),
        })?;

    for (i, &v) in team_keys.iter().enumerate() {
        if task_keys[i] > v {
            return Err(AppError::TaskNotOpenned);
        }
    }
    Ok(())
}

pub fn get_task(task_id: i32, team_id: i32, pool: &Pool) -> Result<TaskFullInfo, AppError> {
    use crate::models::schema::tasks::dsl::*;

    let conn = &pool.get().unwrap();

    let task: Task = tasks.filter(id.eq(&task_id)).first(conn).map_err(|err| {
        if err == diesel::NotFound {
            AppError::TaskNotFound
        } else {
            AppError::ServiceError {
                cause: err.to_string(),
            }
        }
    })?;

    check_access(task_id, team_id, pool)?;

    let solved = check_solved(task_id, team_id, pool)?;

    Ok(TaskFullInfo {
        title_ru: task.title_ru,
        title_en: task.title_en,
        description_ru: task.description_ru,
        description_en: task.description_en,
        points: task.points,
        keys_reward: task.keys_reward,
        keys_condition: task.keys_condition,
        solved,
    })
}

fn check_solved(task: i32, team: i32, pool: &Pool) -> Result<bool, AppError> {
    use crate::models::schema::completed::dsl::*;

    let conn = &pool.get().unwrap();

    diesel::select(diesel::dsl::exists(
        completed
            .filter(task_id.eq(&task))
            .filter(team_id.eq(&team))
            .filter(solved.eq(true)),
    ))
    .get_result::<bool>(conn)
    .map_err(|err| AppError::ServiceError {
        cause: err.to_string(),
    })
}

pub fn push_flag(team: i32, task: i32, flag: &str, pool: &Pool) -> Result<bool, AppError> {
    use crate::models::schema::{completed, tasks, team_game};

    let conn = &pool.get().unwrap();

    check_access(task, team, pool)?;

    let solved = check_solved(task, team, pool)?;

    if solved {
        return Err(AppError::TaskAlreadySolved);
    }

    let task_info: Task = tasks::table
        .filter(tasks::id.eq(task))
        .first(conn)
        .map_err(|err| AppError::ServiceError {
            cause: err.to_string(),
        })?;

    let flag_re = Regex::new(&task_info.flag).unwrap();

    let mut solved = false;

    if flag_re.is_match(&flag) {
        solved = true;

        let (team_keys, team_points): (Vec<i32>, i32) = team_game::table
            .select((team_game::keys_owned, team_game::points))
            .filter(team_game::team_id.eq(team))
            .first(conn)
            .map_err(|err| AppError::ServiceError {
                cause: err.to_string(),
            })?;

        let mut new_keys = team_keys.clone();

        for i in 0..6 {
            if new_keys[i] < task_info.keys_reward[i] {
                new_keys[i] = task_info.keys_reward[i];
            }
        }

        diesel::update(team_game::table.filter(team_game::team_id.eq(team)))
            .set((
                team_game::keys_owned.eq(new_keys),
                team_game::points.eq(team_points + task_info.points),
            ))
            .execute(conn)
            .map_err(|err| AppError::ServiceError {
                cause: err.to_string(),
            })?;
    }

    let push_flag = PushFlag {
        team_id: team,
        task_id: task,
        flag: &flag,
        time: chrono::Local::now().naive_local(),
        solved,
    };

    debug!(
        "Team {} tried to solve task {} (id: {}) with flag: {}",
        team, task_info.title_ru, task, flag
    );

    diesel::insert_into(completed::table)
        .values(&push_flag)
        .execute(conn)
        .map_err(|err| AppError::ServiceError {
            cause: err.to_string(),
        })
        .map(|ret| ret != 0)?;

    Ok(solved)
}

pub fn get_scoreboard(pool: &Pool) -> Result<Vec<Scoreboard>, AppError> {
    use crate::models::schema::{team_game, team_info};

    let conn = &pool.get().unwrap();

    team_game::table
        .inner_join(team_info::table)
        .select((team_info::name, team_game::points))
        .order_by(team_game::points.desc())
        .load::<Scoreboard>(conn)
        .map_err(|err| AppError::ServiceError {
            cause: err.to_string(),
        })
}

pub fn get_team(team: i32, pool: &Pool) -> Result<TeamGameInfo, AppError> {
    use crate::models::schema::{team_game, team_info};

    let conn = &pool.get().unwrap();

    team_game::table
        .inner_join(team_info::table)
        .select((team_info::name, team_game::keys_owned, team_game::points))
        .filter(team_info::id.eq(&team))
        .first::<TeamGameInfo>(conn)
        .map_err(|err| AppError::ServiceError {
            cause: err.to_string(),
        })
}

pub fn load_map(team_id: i32, pool: &Pool) -> Result<Vec<TaskShortInfo>, AppError> {
    use crate::models::schema::tasks::dsl::*;

    let conn = &pool.get().unwrap();

    let loaded_tasks: Vec<Task> = tasks.load(conn).map_err(|err| {
        if err == diesel::NotFound {
            AppError::TaskNotFound
        } else {
            AppError::ServiceError {
                cause: err.to_string(),
            }
        }
    })?;

    let mut short_tasks: Vec<TaskShortInfo> = vec![];

    for task in loaded_tasks {
        let access = check_access(task.id, team_id, &pool).is_ok();

        let solved = check_solved(task.id, team_id, &pool)?;

        short_tasks.push(TaskShortInfo {
            id: task.id,
            title_ru: task.title_ru,
            title_en: task.title_en,
            points: task.points,
            keys_reward: task.keys_reward,
            keys_condition: task.keys_condition,
            access,
            place: task.place,
            solved,
            tags: task.tags,
            character: task.character,
        });
    }

    Ok(short_tasks)
}
