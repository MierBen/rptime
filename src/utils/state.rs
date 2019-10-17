use crate::database::Pool;
use chrono::NaiveDateTime;
use std::sync::{Arc, Mutex};

pub struct AppData {
    pub start_game: Mutex<NaiveDateTime>,
    pub end_game: Mutex<NaiveDateTime>,
    pub pool: Pool,
}
