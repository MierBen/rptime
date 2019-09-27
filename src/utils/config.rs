use std::fs::read_to_string;
use toml::from_str;
use serde::Deserialize;
use failure::Fallible;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub game: GameConfig,
    pub log: LogConfig,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn from_file(file: &str) -> Fallible<Self> {
        Ok(from_str(&read_to_string(file)?)?)
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {

    pub url: String,

    pub secret_key: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GameConfig {

    pub start_game: String,

    pub end_game: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LogConfig {

    pub rptime: String,

    pub actix_web: String,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,

    pub username: String,

    pub password: String,

    pub database: String,
}