use crate::{
    api::{
        auth::{login, logout, register},
        game::{index, map_config, me, scoreboard, solve, task},
    },
    database::{import_tasks, init_db, init_game},
    middleware::{CheckGame, CheckLogin},
    utils::{load_tasks_from_path, load_tasks_from_repo, AppData, Config},
};
use actix::SystemRunner;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{guard, middleware, web, App, HttpServer};
use chrono::NaiveDateTime;
use failure::Fallible;
use std::{sync::Mutex, thread};

pub struct Server {
    cfg: Config,
    runner: SystemRunner,
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

        let start_game =
            NaiveDateTime::parse_from_str(&config.game.start_game, "%Y-%m-%d %H:%M:%S").unwrap();
        let end_game =
            NaiveDateTime::parse_from_str(&config.game.end_game, "%Y-%m-%d %H:%M:%S").unwrap();

        init_game((start_game, end_game), &pool);

        let (tasks, map) = if let Some(url) = &config.game.url {
            load_tasks_from_repo(url, &config.game.path)
        } else {
            load_tasks_from_path(&config.game.path)
        }?;

        let count = import_tasks(tasks, &pool)?;
        info!("Inserted {} tasks", count);

        let secret_key = config.server.secret_key.clone();

        let server = HttpServer::new(move || {
            App::new()
                .data(AppData {
                    start_game: Mutex::new(start_game),
                    end_game: Mutex::new(end_game),
                    pool: pool.clone(),
                })
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(secret_key.as_bytes())
                        .name("s")
                        .secure(false),
                ))
                .wrap(middleware::Logger::default())
                .service(
                    web::scope("/api/v1/")
                        .guard(guard::Header("Content-Type", "application/json"))
                        .service(
                            web::resource("/register")
                                .wrap(CheckGame)
                                .route(web::post().to_async(register)),
                        )
                        .service(
                            web::resource("/login")
                                .wrap(CheckGame)
                                .route(web::post().to_async(login)),
                        )
                        .service(
                            web::resource("/logout")
                                .wrap(CheckGame)
                                .wrap(CheckLogin)
                                .route(web::post().to_async(logout)),
                        )
                        .service(
                            web::resource("/map")
                                .wrap(CheckGame)
                                .wrap(CheckLogin)
                                .route(web::get().to_async(index)),
                        )
                        .service(
                            web::resource("/scoreboard")
                                .route(web::get().to_async(scoreboard)),
                        )
                        .service(
                            web::resource("/task/{task_id}")
                                .wrap(CheckGame)
                                .wrap(CheckLogin)
                                .route(web::get().to_async(task)),
                        )
                        .service(
                            web::resource("/task/{task_id}/flag")
                                .wrap(CheckGame)
                                .wrap(CheckLogin)
                                .route(web::post().to_async(solve)),
                        )
                        .service(
                            web::resource("/me")
                                .wrap(CheckGame)
                                .wrap(CheckLogin)
                                .route(web::get().to_async(me)),
                        )
                        .service(
                            web::resource("/map_config")
                                .data(map.clone())
                                .wrap(CheckGame)
                                .wrap(CheckLogin)
                                .route(web::get().to_async(map_config)),
                        ),
                )
        });

        let url = config.server.url.clone();

        server.bind(&url)?.start();

        Ok(Self {
            cfg: config.to_owned(),
            runner,
        })
    }

    pub fn run(self) -> Fallible<()> {
        self.check_map();

        self.runner.run()?;
        Ok(())
    }

    fn check_map(&self) {
        thread::spawn(move || unimplemented!());
    }
}
