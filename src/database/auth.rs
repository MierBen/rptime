use super::Pool;
use crate::{
    models::{Login, NewTeam, Register, SetTeamGameInfo, Team},
    utils::AuthError,
};
use diesel::{prelude::*, RunQueryDsl};
use regex::Regex;
use uuid::Uuid;

pub fn register_query(data: Register, pool: &Pool) -> Result<Team, AuthError> {
    use crate::models::schema::team_game;
    use crate::models::schema::team_info::dsl::*;

    let conn = &pool.get().unwrap();

    lazy_static! {
        static ref MAIL: Regex = Regex::new(r"\w+@\w+\.\w+").unwrap();
    }

    if !MAIL.is_match(&data.email) {
        return Err(AuthError::InvalidEmail);
    }

    if data.team_name.len() < 3 && data.team_name.len() > 40 {
        return Err(AuthError::FieldEmpty {
            field: "team name".to_string(),
        });
    }

    let check_exist = team_info
        .filter(name.eq(&data.team_name))
        .or_filter(email.eq(&data.email))
        .first::<Team>(conn);

    if let Err(diesel::NotFound) = check_exist {
        let mut tok = Uuid::new_v4().to_string();
        tok.retain(|c| c != '-');

        let team = NewTeam {
            name: &data.team_name.trim(),
            email: &data.email.trim(),
            country: &data.country.trim(),
            university: &data.university.trim(),
            token: &tok,
        };

        debug!("For team {} created token: {}", team.name, team.token,);

        let team: Team = diesel::insert_into(team_info)
            .values(&team)
            .get_result(conn)
            .map_err(|err| AuthError::ServiceError {
                cause: err.to_string(),
            })?;

        let keys = vec![1; 6];

        let team_game_info = SetTeamGameInfo {
            team_id: team.id,
            keys_owned: keys,
            points: 0,
        };

        diesel::insert_into(team_game::table)
            .values(&team_game_info)
            .execute(conn)
            .map_err(|err| AuthError::ServiceError {
                cause: err.to_string(),
            })?;

        Ok(team)
    } else {
        Err(AuthError::TeamExist)
    }
}

pub fn login_query(data: Login, pool: &Pool) -> Result<Team, AuthError> {
    use crate::models::schema::team_info::dsl::*;

    let conn = &pool.get().unwrap();

    team_info
        .filter(token.eq(&data.token))
        .first::<Team>(conn)
        .map_err(|err| {
            if err == diesel::NotFound {
                AuthError::BadToken
            } else {
                AuthError::ServiceError {
                    cause: err.to_string(),
                }
            }
        })
}
