use diesel::{prelude::*, r2d2::{self, ConnectionManager}, PgConnection, RunQueryDsl};
use uuid::Uuid;
use actix_web::web::Data;
use regex::Regex;

use crate::{
    models::{Register, Login, Team, NewTeam },
    utils::{
        AuthError,
        AppError
    },
};
use chrono::{
    NaiveDateTime
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn register_query(
    data: Register,
    pool: Data<Pool>
) -> Result<Team, AuthError> {
    use crate::models::schema::team_info::dsl::*;

    let conn = &pool.get().unwrap();

    lazy_static! {
        static ref MAIL: Regex = Regex::new(r"\w+@\w+\.\w+").unwrap();
    }

    if !MAIL.is_match(&data.email) {
        return Err(AuthError::InvalidEmail);
    }

    let check_exist = team_info
        .filter(name.eq(&data.team_name))
        .or_filter(email.eq(&data.email))
        .first::<Team>(conn);

    if let Err(diesel::NotFound) = check_exist {

        let tok = &Uuid::new_v4().to_string();

        let team = NewTeam {
            name: &data.team_name,
            email: &data.email,
            country: &data.country,
            university: &data.university,
            token: tok,
        };

        debug!("For team {} created token: {}",
               team.name,
               team.token,
        );

        diesel::insert_into(team_info)
            .values(&team)
            .get_result::<Team>(conn)
            .map_err(|err| AuthError::ServiceError { cause: err.to_string() })
    } else {
        Err(AuthError::TeamExist)
    }
}

pub fn login_query(
    data: Login,
    pool: Data<Pool>
) -> Result<Team, AuthError> {
    use crate::models::schema::team_info::dsl::*;

    let conn = &pool.get().unwrap();

    team_info
        .filter(token.eq(&data.token))
        .first::<Team>(conn)
        .map_err(|err| if err == diesel::NotFound {
            AuthError::BadToken
        } else {
            AuthError::ServiceError { cause: err.to_string() }
        })
}

pub fn init_game(
    (start, end): (NaiveDateTime, NaiveDateTime),
    pool: &Pool,
) -> Result<(), AppError> {

    use crate::models::schema::game::dsl::*;

    let conn = &pool.get().unwrap();

    let res = diesel::insert_into(game)
        .values((&start_game.eq(start), &end_game.eq(end)))
        .execute(conn);

    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(AppError::ServiceError { cause : err.to_string() })
    }
}

pub fn check_game(
    pool: Data<Pool>
) -> Result<String, AppError> {

    use crate::models::schema::game::dsl::*;

    let conn = &pool.get().unwrap();

    let (start_time, end_time) = game
        .select((start_game, end_game))
        .order(id.desc())
        .first::<(NaiveDateTime, NaiveDateTime)>(conn)
        .map_err(| err | AppError::ServiceError { cause: err.to_string() })?;

    let now = chrono::Local::now()
        .naive_local();

    if now < start_time {
        Err(AppError::GameNotStarted)
    } else if now > end_time {
        Err(AppError::GameOver)
    } else {
        Ok(String::from("Context running"))
    }
}

pub fn init_db(database_url: String) -> Pool {
    let manager =
        ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).unwrap()
}