#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

mod api;
mod database;
mod middleware;
mod models;
pub mod server;
pub mod utils;

pub use server::Server;
pub use utils::{logger_init, Config};
