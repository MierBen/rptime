use std::env::set_var;
use failure::Fallible;
use rptime_backend::{Server, Config};
use clap::{App, crate_version, crate_authors};
use log::info;

fn main() -> Fallible<()> {

    let matches = App::new("rptime-backend")
        .version(crate_version!())
        .author(crate_authors!())
        .about("The backend web server for CTF")
        .args_from_usage(
            "-c, --config=[FILE] 'Set custom config file'")
        .get_matches();


    let config_file = matches.value_of("config").unwrap_or("Config.toml");

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
