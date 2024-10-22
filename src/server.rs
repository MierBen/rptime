use crate::{
    api::{
        auth::{login, logout, register},
        game::{index, map_config, me, scoreboard, solve, task},
    },
    database::{import_tasks, init_db},
    middleware::CheckAuthService,
    utils::{load_tasks_from_path, load_tasks_from_repo, AppData, Config},
};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use chrono::NaiveDateTime;
use failure::Fallible;

pub async fn server(config: Config) -> std::io::Result<()> {
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

    let (tasks, map) = if let Some(url) = &config.game.url {
        load_tasks_from_repo(url, &config.game.path)
    } else {
        load_tasks_from_path(&config.game.path)
    }
    .unwrap();

    let count = import_tasks(tasks, &pool).expect("Can't import tasks");
    info!("Inserted {} tasks", count);

    let secret_key = config.server.secret_key.clone();

    let server = HttpServer::new(move || {
        App::new()
            .data(AppData {
                start_game,
                end_game,
                pool: pool.clone(),
            })
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(secret_key.as_bytes())
                    .name("s")
                    .secure(false),
            ))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1/auth")
                    .service(
                        web::resource("/register")
                            // .wrap(CheckGame)
                            .route(web::post().to(register)),
                    )
                    .service(
                        web::resource("/login")
                            // .wrap(CheckGame)
                            .route(web::post().to(login)),
                    )
                    .service(
                        web::resource("/logout")
                            // .wrap(CheckGame)
                            .route(web::post().to(logout)),
                    ),
            )
            .service(
                web::scope("/api/v1/game")
                    .wrap(CheckAuthService)
                    .service(
                        web::resource("/")
                            // .wrap(CheckGame)
                            .route(web::get().to(index)),
                    )
                    .service(
                        web::resource("/task/{task_id}")
                            // .wrap(CheckGame)
                            .route(web::get().to(task)),
                    )
                    .service(
                        web::resource("/task/{task_id}/flag")
                            // .wrap(CheckGame)
                            .route(web::post().to(solve)),
                    )
                    .service(
                        web::resource("/map_config")
                            .data(map.clone())
                            // .wrap(CheckGame)
                            .route(web::get().to(map_config)),
                    ),
            )
            .service(
                web::scope("/api/v1/team")
                    .service(
                        web::resource("/me")
                            // .wrap(CheckGame)
                            .route(web::get().to(me)),
                    )
                    .service(web::resource("/scoreboard").route(web::get().to(scoreboard))),
            )
    })
    .bind(&config.server.url)?;

    server.run().await
}
