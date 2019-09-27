use actix::SystemRunner;
use actix_web::{middleware, guard, web, HttpServer, App};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use chrono::NaiveDateTime;
use failure::Fallible;
use crate::{
    utils::{
        Config,
    },
    database::{
        init_db,
        init_game,
    },
    api::{
        auth::{
            login,
            logout,
            register,
        },
        game::{
            index,
        }
    },
    middleware::{
        CheckLogin,
        CheckGame
    },
};


pub struct Server {
    cfg: Config,
    runner: SystemRunner,
    url: String,
}

impl Server {
    pub fn from_config(config: &Config) -> Fallible<Self> {
        let runner = actix::System::new("rptime-backend");

        let database_url = format!(
            "postgres://{}:{}@{}/{}",
            config.database.username,
            config.database.password,
            config.database.host,
            config.database.database,
        );

        let pool = init_db(database_url);

        let start_game = NaiveDateTime::parse_from_str(
            &config.game.start_game, "%Y-%m-%d %H:%M:%S").unwrap();
        let end_game = NaiveDateTime::parse_from_str(
            &config.game.end_game, "%Y-%m-%d %H:%M:%S").unwrap();

        let _ = init_game(
            (start_game, end_game), &pool
        ).map_err(|err| panic!("Error while initializing game! Error: {:?}", err));

        let secret_key = config.server.secret_key.clone();

        let server = HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(
                        secret_key.as_bytes()
                    ).name("s")
                        .secure(false)
                ))
                .wrap(middleware::Logger::default())
                .service(
                    web::scope("/api/v1/")
                        .guard(guard::Header("content-type", "application/json"))
                        .service(
                            web::resource("/register")
                                .wrap(CheckGame)
                                .route(web::post().to_async(register))
                        )
                        .service(
                            web::resource("/login")
                                .wrap(CheckGame)
                                .route(web::post().to_async(login))
                        )
                        .service(
                            web::resource("/logout")
                                .wrap(CheckGame)
                                .route(web::post().to_async(logout))
                        )
                        .service(
                            web::resource("/map")
                                .wrap(CheckGame)
                                .wrap(CheckLogin)
                                .route(web::get().to_async(index))
                        )
            )
        });

        let url = config.server.url.clone();

        server.bind(&url)?.start();

        Ok(Self {
            cfg: config.to_owned(),
            runner,
            url,
        })

    }

    pub fn run(self) -> Fallible<()>{
        self.runner.run()?;
        Ok(())
    }
}