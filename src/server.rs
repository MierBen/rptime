use actix::SystemRunner;
use actix_web::{middleware, guard, web, HttpServer, App};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use failure::Fallible;
use crate::{
    utils::{
        Config,
    },
    database::init_db,
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
    middleware::CheckLogin,
};


pub struct Server {
    cfg: Config,
    runner: SystemRunner,
    url: String,
}

impl Server {
    pub fn from_config(config: &Config) -> Fallible<Self> {
        let runner = actix::System::new("rptime");

        let database_url = format!(
            "postgres://{}:{}@{}/{}",
            config.database.username,
            config.database.password,
            config.database.host,
            config.database.database,
        );

        let pool = init_db(database_url);

        let server = HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(
                        "9bg1Ujcuz89wGCQTgBThiuJQzmEr7xIp".as_bytes()
                    ).name("s")
                        .secure(false)
                ))
                .wrap(middleware::Logger::default())
                .service(
                    web::scope("/api/v1/")
                        .guard(guard::Header("content-type", "application/json"))
                        .service(
                            web::resource("/register")
                                .route(web::post().to_async(register))
                        )
                        .service(
                            web::resource("/login")
                                .route(web::post().to_async(login))
                        )
                        .service(
                            web::resource("/logout")
                                .route(web::post().to_async(logout))
                        )
                        .service(
                            web::resource("/map")
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