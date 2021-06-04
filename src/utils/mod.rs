use log::LevelFilter;
use std::fs::File;

mod config;
mod error;
mod state;
mod tasker;

pub use config::Config;
pub use error::{AppError, AuthError, ResponseJsonError};
pub use state::AppData;
pub use tasker::{load_tasks_from_path, load_tasks_from_repo, map_getter, Map};

pub fn logger_init(config: &Config) {
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            config.log.terminal,
            simplelog::ConfigBuilder::new()
                .set_time_level(LevelFilter::Info)
                .set_time_to_local(true)
                .set_time_format_str("%Y-%m-%d %H:%M:%S")
                .add_filter_allow_str("actix-web")
                .add_filter_allow_str("rptime")
                .build(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        simplelog::WriteLogger::new(
            config.log.file,
            simplelog::ConfigBuilder::new()
                .set_time_level(LevelFilter::Info)
                .set_time_to_local(true)
                .set_time_format_str("%Y-%m-%d %H:%M:%S")
                .add_filter_allow_str("actix-web")
                .add_filter_allow_str("rptime")
                .build(),
            File::create(&config.log.log_file).unwrap(),
        ),
    ])
    .unwrap();
}
