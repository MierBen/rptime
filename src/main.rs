use std::env::set_var;
use failure::Fallible;
use rptime_backend::{Server, Config};
use clap::App;
use log::info;

fn main() -> Fallible<()> {

    App::new("RpTime");

    let config_file = "Config.toml";

    let config = Config::from_file(config_file)?;

    set_var("RUST_LOG",
    format!(
        "actix_web={},rptime={}",
        config.log.actix_web, config.log.rptime
    ));

    env_logger::init();

    let server = Server::from_config(&config)?;

    info!(
        "Starting server from config path {} for url {}",
        config_file, config.server.url
    );
    server.run()?;

    Ok(())
}
