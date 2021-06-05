#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use backend::auth::auth0::{AuthParameters, SessionDB};
use backend::db::operations::BlockplotDbConn;

use dashmap::DashMap;

use rocket::fairing::AdHoc;

use rocket_contrib::templates::Template;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        //TODO: Swtich to more strict options
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()?;
    
    let sessions = SessionDB(DashMap::new());
    
    rocket::ignite()
        .attach(BlockplotDbConn::fairing())
        .attach(cors)
        .attach(Template::fairing())
        .mount("/", routes![auth0_login, home, index, get_skillblocks, get_skillblocks_redirect, process_login, process_logout, test_post])
        .manage(sessions)
        .attach(AdHoc::on_attach("Parameters Config", |rocket| {
            let config = rocket.config();
            let auth_parameters = AuthParameters::new(config).unwrap();

            Ok(rocket.manage(auth_parameters))
        }))
        .launch();

    Ok(())
}