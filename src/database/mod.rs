use diesel::{prelude::*, r2d2::{self, ConnectionManager}, PgConnection, RunQueryDsl};
use crate::models::models::{Register, Login, Team, NewTeam };
use uuid::Uuid;
use actix_web::web::Data;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DatabaseExecutor(pub Pool);

pub fn register_query(
    data: Register,
    pool: Data<Pool>
) -> Result<Team, diesel::result::Error> {
    use crate::models::schema::team_info::dsl::*;

    let conn = &pool.get().unwrap();

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

    let check_exist = team_info
        .filter(name.eq(&team.name))
        .or_filter(email.eq(&team.email))
        .first::<Team>(conn);

    if let Err(diesel::NotFound) = check_exist {
        diesel::insert_into(team_info)
            .values(&team)
            .get_result::<Team>(conn)
    } else {
        Err(diesel::result::Error::AlreadyInTransaction) // Изменить
    }
}

pub fn login_query(
    data: Login,
    pool: Data<Pool>
) -> Result<Team, diesel::result::Error> {
    use crate::models::schema::team_info::dsl::*;

    let conn = &pool.get().unwrap();

    team_info
        .filter(token.eq(&data.token))
        .first::<Team>(conn)
}


pub fn init_db(database_url: String) -> Pool {
    let manager =
        ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).unwrap()
}