use failure::Fallible;
use log::LevelFilter;
use serde::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

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

    pub path: String,

    pub url: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LogConfig {
    pub terminal: LevelFilter,

    pub file: LevelFilter,

    pub log_file: String,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,

    pub username: String,

    pub password: String,

    pub database: String,
}
