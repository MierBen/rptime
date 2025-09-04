use serde::{Deserialize, Serialize};
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
