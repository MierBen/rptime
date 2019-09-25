#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;


pub mod server;
pub mod utils;
mod database;
mod models;
mod api;

pub use server::Server;
pub use utils::Config;