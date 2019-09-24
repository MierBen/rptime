use actix::{SystemRunner};
use actix_web::{middleware, web, HttpServer, HttpResponse, Error, App};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use futures::{Future, future::ok as fut_ok};
use failure::Fallible;
use crate::{
    config::Config,
    database::{
        Pool,
        init_db,
        login_query,
        register_query,
    },
    models::{
        Login,
        Register
    }
};

fn register(
    data: web::Json<Register>,
    pool: web::Data<Pool>
) -> impl Future<Item=HttpResponse, Error=Error> {
    debug!("Got team_name: {}\nemail: {}\ncountry: {}",
           data.team_name, data.email, data.country);

    web::block(move || register_query(data.into_inner(), pool))
        .then(|res | match res {
            Ok(team) => {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(team))
            },
            Err(_) => {
                Ok(HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json("Team already exist"))
            }
        })
}

fn login(
    data: web::Json<Login>,
    id: Identity,
    pool: web::Data<Pool>
) -> impl Future<Item = HttpResponse, Error = Error> {
    debug!("Got token: {}",
           data.token);
    web::block(move || login_query(data.into_inner(), pool))
        .then(|res| match res {
            Ok(team) => {
//                    id.remember(team.id.clone().to_string());
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json("Success"))
            },
            Err(_) => {
                Ok(HttpResponse::NotFound().finish())
            }
        })
}

fn logout(id: Identity) -> impl Future<Item = HttpResponse, Error = Error> {

    if let Some(_token) = id.identity() {
        id.forget();
    }
    fut_ok(HttpResponse::Ok()
        .content_type("application/json")
        .json("Success"))
}


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