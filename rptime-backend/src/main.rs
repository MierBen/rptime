use clap::{Command, arg};
use failure::Fallible;
use log::info;
use rptime_backend::{server, Config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = Command::new("rptime-backend")
        .version("1.0")
        .about("The rptime-backend web server for CTF")
        .arg(arg!(-c --config <FILE> "Set custom config file").required(true))
        .get_matches();

    let config_file = matches.get_one::<String>("config").expect("required");

    let config = Config::from_file(config_file).unwrap();

    info!(
        "Starting server from config path {} for url {}",
        config_file, config.server.url
    );

    server(config).await
}
