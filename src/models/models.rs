use super::schema::{completed, tasks, team_game, team_info};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub country: Option<String>,
    pub university: Option<String>,
    pub token: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "team_info"]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub country: &'a str,
    pub university: &'a str,
    pub token: &'a str,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct TeamGameInfo {
    pub team_name: String,
    pub keys_owned: Vec<i32>,
    pub points: i32,
}

#[derive(Insertable, Serialize)]
#[table_name = "team_game"]
pub struct SetTeamGameInfo {
    pub team_id: i32,
    pub keys_owned: Vec<i32>,
    pub points: i32,
}

#[derive(Deserialize)]
pub struct Login {
    pub token: String,
}

#[derive(Deserialize)]
pub struct Register {
    pub team_name: String,
    pub email: String,
    pub country: String,
    pub university: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub title_ru: String,
    pub title_en: Option<String>,
    pub description_ru: String,
    pub description_en: Option<String>,
    pub flag: String,
    pub points: i32,
    pub keys_reward: Vec<i32>,
    pub keys_condition: Vec<i32>,
    pub place: i32,
    pub author: String,
    pub character: i32,
    pub tags: String,
}

#[derive(Serialize, Insertable)]
#[table_name = "tasks"]
pub struct InsertTask {
    pub title_ru: String,
    pub title_en: Option<String>,
    pub description_ru: String,
    pub description_en: Option<String>,
    pub flag: String,
    pub points: i32,
    pub keys_reward: Vec<i32>,
    pub keys_condition: Vec<i32>,
    pub place: i32,
    pub author: String,
    pub character: i32,
    pub tags: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskShortInfo {
    pub id: i32,
    pub title_ru: String,
    pub title_en: Option<String>,
    pub points: i32,
    pub keys_reward: Vec<i32>,
    pub keys_condition: Vec<i32>,
    pub access: bool,
    pub place: i32,
    pub solved: bool,
    pub tags: String,
    pub character: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TaskFullInfo {
    pub title_ru: String,
    pub title_en: Option<String>,
    pub description_ru: String,
    pub description_en: Option<String>,
    pub points: i32,
    pub keys_reward: Vec<i32>,
    pub keys_condition: Vec<i32>,
    pub solved: bool,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Scoreboard {
    pub team_name: String,
    pub points: i32,
}

#[derive(Deserialize)]
pub struct GetFlag {
    pub flag: String,
}

#[derive(Insertable)]
#[table_name = "completed"]
pub struct PushFlag<'a> {
    pub team_id: i32,
    pub task_id: i32,
    pub time: NaiveDateTime,
    pub flag: &'a str,
    pub solved: bool,
}
