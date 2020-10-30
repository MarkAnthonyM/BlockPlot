#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use backend::db::models;

use dotenv::dotenv;
use std::env;

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

use rusty_rescuetime::analytic_data::AnalyticData;
use rusty_rescuetime::parameters::Parameters;
use rusty_rescuetime::parameters::PerspectiveOptions::{ Interval, Rank };
use rusty_rescuetime::parameters::ResolutionOptions::Day;
use rusty_rescuetime::parameters::RestrictData::{ Date, Thing };
use rusty_rescuetime::parameters::RestrictOptions::Overview;

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

//TODO: Figure out if time data should be restructured in a different format for the frontend
//TODO: Explore whether a hashmap of time data should be processed and returned as json
#[get("/api/v1/categories/software_development")]
fn get_categories() -> Json<AnalyticData> {
    dotenv().ok();

    let api_key = env::var("API_KEY").unwrap();
    let format = String::from("json");

    let query_parameters = Parameters::new(
        Some(Interval),
        Some(Day),
        Some(Date("2020-09-23", "2020-10-23")),
        Some(Overview),
        Some(Thing("software development")),
        None,
    );

    let response = AnalyticData::fetch(&api_key, query_parameters, format);

    Json(response.unwrap())
}

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
    
    rocket::ignite()
        .attach(BlockplotDbConn::fairing())
        .attach(cors)
        .mount("/", routes![get_times, get_categories])
        .launch();

    Ok(())
}