use super::schema::team_info;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub country: Option<String>,
    pub university: Option<String>,
    pub token: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "team_info"]
pub struct NewTeam <'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub country: &'a str,
    pub university: &'a str,
    pub token: &'a str,
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