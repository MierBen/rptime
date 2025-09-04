use crate::{
    api::{
        auth::{login, logout, register},
        game::{index, map_config, me, scoreboard, solve, task},
    },
    database::{tasks::import_tasks, init_db},
    middleware::CheckAuthService,
    utils::{load_tasks_from_path, load_tasks_from_repo, AppData, Config},
};

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{middleware, web, App, HttpServer, cookie::Key};
use chrono::NaiveDateTime;

pub async fn server(config: Config) -> std::io::Result<()> {
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.database,
    );

    let secret_key = Key::from(config.server.secret_key.as_bytes());
    let pool = init_db(database_url);

    let start_game =
        NaiveDateTime::parse_from_str(&config.game.start_game, "%Y-%m-%d %H:%M:%S").expect("Corrected start game time expected");
    let end_game =
        NaiveDateTime::parse_from_str(&config.game.end_game, "%Y-%m-%d %H:%M:%S").expect("Corrected end game time expected");

    let (tasks, map) = if let Some(url) = &config.game.url {
        load_tasks_from_repo(url, &config.game.path)
    } else {
        load_tasks_from_path(&config.game.path)
    }
    .unwrap();

    let count = import_tasks(tasks, &pool).expect("Can't import tasks");
    info!("Inserted {} tasks", count);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(AppData {
                start_game,
                end_game,
                pool: pool.clone(),
            })
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone(),
                )
                    .cookie_name("crf-key".to_string())
                    .cookie_http_only(true)
                    .build()
            )
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1/auth")
                    .service(register)
                    .service(login)
                    .service(logout)
            )
            .service(
                web::scope("/api/v1/game")
                    .wrap(CheckAuthService)
                    .service(index)
                    .service(task)
                    .service(solve)
                    .service(map_config)
            )
            .service(
                web::scope("/api/v1/team")
                    .service(me)
                    .service(scoreboard)
            )
    })
    .bind(&config.server.url)?;

    server.run().await
}
