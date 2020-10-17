#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use dotenv::dotenv;
use std::env;

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

use rusty_rescuetime::analytic_data::AnalyticData;
use rusty_rescuetime::parameters::Parameters;
use rusty_rescuetime::parameters::PerspectiveOptions::Rank;

// Rocket connection pool
#[database("postgres_blockplot")]
struct BlockplotDbConn(diesel::PgConnection);

// rusty-rescuetime api testing route
#[get("/times")]
fn get_times() -> Json<AnalyticData> {
    dotenv().ok();
    
    let api_key = env::var("API_KEY").unwrap();
    let format = String::from("json");
    
    let query_parameters = Parameters::new(
        Some(Rank),
        None,
        None,
        None,
        None,
        None,
    );

    let response = AnalyticData::fetch(&api_key, query_parameters, format);

    Json(response.unwrap())
}

fn main() {
    rocket::ignite()
        .attach(BlockplotDbConn::fairing())
        .mount("/", routes![get_times])
        .launch();
}