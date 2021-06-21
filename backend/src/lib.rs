#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

use crate::auth::auth0::{AuthParameters, SessionDB, Settings};
use crate::db::operations::BlockplotDbConn;

use dashmap::DashMap;

use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use rocket_contrib::templates::Template;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use std::collections::HashMap;
use std::net::TcpListener;

pub mod auth;
pub mod configuration;
pub mod db;
pub mod routes;

pub fn rocket(
    testing: bool,
    listener: Option<TcpListener>,
    db_config: Option<HashMap<&str, Value>>,
) -> rocket::Rocket {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        //TODO: Swtich to more strict options
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors();

    let sessions = SessionDB(DashMap::new());

    let rocket: rocket::Rocket;

    // Setup rocket instance based on whether running integrations tests or not
    if testing {
        let address_config = listener.unwrap();
        let port = address_config.local_addr().unwrap().port();
        let config = Config::build(Environment::Development)
            .address("127.0.0.1")
            .port(port)
            .extra("databases", db_config.unwrap())
            .finalize();
        rocket = rocket::custom(config.unwrap()).attach(AdHoc::on_attach(
            "Parameters Config",
            |rocket| {
                let settings = Settings::new().unwrap();
                let auth_parameters = AuthParameters::new_testing(settings).unwrap();

                Ok(rocket.manage(auth_parameters))
            },
        ));
    } else {
        rocket = rocket::ignite()
            .attach(Template::fairing())
            .attach(AdHoc::on_attach("Parameters Config", |rocket| {
                let config = rocket.config();
                let auth_parameters = AuthParameters::new(config).unwrap();

                Ok(rocket.manage(auth_parameters))
            }));
    }

    rocket
        .attach(cors.unwrap())
        .attach(BlockplotDbConn::fairing())
        .mount(
            "/",
            routes![
                routes::authentication::auth0_login,
                routes::authentication::process_login,
                routes::authentication::process_logout,
                routes::health::health_check,
                routes::index::home,
                routes::index::index,
                routes::skillblocks::get_skillblocks,
                routes::skillblocks::get_skillblocks_redirect,
                routes::skillblocks::new_skillblock,
                routes::skillblocks::new_skillblock_redirect,
            ],
        )
        .manage(sessions)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
