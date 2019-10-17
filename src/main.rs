use clap::{crate_authors, crate_version, App};
use failure::Fallible;
use log::info;
use rptime_backend::{logger_init, Config, Server};

fn main() -> Fallible<()> {
    let matches = App::new("rptime-backend")
        .version(crate_version!())
        .author(crate_authors!())
        .about("The backend web server for CTF")
        .args_from_usage("-c, --config=[FILE] 'Set custom config file'")
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("Config.toml");

    let config = Config::from_file(config_file)?;

    logger_init(&config);

    let server = Server::from_config(&config)?;

    info!(
        "Starting server from config path {} for url {}",
        config_file, config.server.url
    );
    server.run()?;

    Ok(())
}
