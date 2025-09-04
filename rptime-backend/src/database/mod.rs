use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

// TODO: Needed full rework for this module

pub mod auth;
pub mod game;
pub mod tasks;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db(database_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).unwrap()
}
