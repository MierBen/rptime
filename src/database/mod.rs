use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection, RunQueryDsl,
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod auth;
mod game;

use crate::{models::InsertTask, utils::AppError};
pub use auth::*;
pub use game::*;

pub fn import_tasks(tasks: Vec<InsertTask>, pool: &Pool) -> Result<usize, AppError> {
    use crate::models::schema::tasks;

    let conn = &pool.get().unwrap();
    info!("Add {} to database", tasks.len());
    diesel::insert_into(tasks::table)
        .values(&tasks)
        .execute(conn)
        .map_err(|err| AppError::ServiceError {
            cause: format!("Error from import_task() - {:?}", err),
        })
}

pub fn init_db(database_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).unwrap()
}
