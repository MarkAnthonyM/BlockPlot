#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

use crate::auth::auth0::{AuthParameters, SessionDB};
use crate::db::operations::BlockplotDbConn;

use dashmap::DashMap;

use rocket::fairing::AdHoc;
use rocket_contrib::templates::Template;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

pub mod auth;
pub mod db;
pub mod routes;

pub fn rocket() -> rocket::Rocket {
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
    
    rocket::ignite()
        .attach(BlockplotDbConn::fairing())
        .attach(cors.unwrap())
        .attach(Template::fairing())
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
                routes::skillblocks::test_post,
            ],
        )
        .manage(sessions)
        .attach(AdHoc::on_attach("Parameters Config", |rocket| {
            let config = rocket.config();
            let auth_parameters = AuthParameters::new(config).unwrap();

            Ok(rocket.manage(auth_parameters))
        }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
