use std::fs::File;
use log::LevelFilter;

use crate::Config;


fn build_logger() -> Result<simplelog::Config, &mut simplelog::ConfigBuilder> {
    simplelog::ConfigBuilder::new()
        .set_time_level(LevelFilter::Info)
        .set_time_offset_to_local()
        .map(|logging_config_builder| {
            logging_config_builder
                .set_time_format_rfc2822()
                .add_filter_allow_str("actix-web")
                .add_filter_allow_str("rptime")
                .build()
        })
}
pub fn init_logger(config: &Config) {
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            config.log.terminal,
            build_logger(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        simplelog::WriteLogger::new(
            config.log.file,
            build_logger(),
            File::create(&config.log.log_file).unwrap(),
        ),
    ])
    .unwrap();
}
