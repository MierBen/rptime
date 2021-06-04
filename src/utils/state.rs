use crate::database::Pool;
use chrono::NaiveDateTime;

pub struct AppData {
    pub start_game: NaiveDateTime,
    pub end_game: NaiveDateTime,
    pub pool: Pool,
}
