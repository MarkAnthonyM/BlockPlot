#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use backend::db::models;

use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

use rocket::request::Form;

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

use rusty_rescuetime::analytic_data::{ AnalyticData, QueryKind };
use rusty_rescuetime::parameters::Parameters;
use rusty_rescuetime::parameters::PerspectiveOptions::Interval;
use rusty_rescuetime::parameters::ResolutionOptions::Day;
use rusty_rescuetime::parameters::RestrictData::{ Date, Thing };
use rusty_rescuetime::parameters::RestrictOptions::{ Category, Overview };

// Rocket connection pool
#[database("postgres_blockplot")]
struct BlockplotDbConn(diesel::PgConnection);

#[derive(FromForm)]
struct Dates {
    begin_date: String,
    end_date: String,
}

//TODO: Figure out if time data should be restructured in a different format for the frontend
//TODO: Explore whether a hashmap of time data should be processed and returned as json
#[get("/api/categories/<category>?<dates..>")]
fn get_categories(category: String, dates: Form<Dates>) -> Json<models::TimeData> {
    dotenv().ok();
    
    let api_key = env::var("API_KEY").unwrap();
    let format = String::from("json");
    
    let query_parameters = Parameters::new(
        Some(Interval),
        Some(Day),
        //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
        Some(Date(dates.begin_date.clone(), dates.end_date.clone())),
        Some(Overview),
        Some(Thing(category)),
        None,
    );

    let payload = AnalyticData::fetch(&api_key, query_parameters, format).unwrap();

    let mut response = models::TimeData {
        category: String::from("software_development"),
        time_data: HashMap::new(),
    };
    
    for query in payload.rows {
        if let QueryKind::SizeSixString(value) = query {
            if let Some(x) = response.time_data.get_mut(&value.perspective) {
                *x += value.time_spent;
            } else {
                response.time_data.insert(value.perspective, value.time_spent);
            }
        }
    }

    Json(response)
}

// Test handler for multiple data requests
#[get("/api/categories/multi")]
fn get_multi() -> Json<models::TimeWrapper> {
    dotenv().ok();
    
    let api_key = env::var("API_KEY").unwrap();
    let format = String::from("json");
    
    let mut time_vec = Vec::new();
    // let mock_categories = vec!["software%20development", "piano%20practice"];
    // let mock_categories = vec![String::from("software%20development"), String::from("piano%20practice")];
    let mock_categories = vec!["software_development", "piano_practice"];

    for item in mock_categories {
        match item {
            "software_development" => {
                let query_parameters = Parameters::new(
                    Some(Interval),
                    Some(Day),
                    //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                    Some(Date(String::from("2019-11-05"), String::from("2020-11-04"))),
                    Some(Overview),
                    Some(Thing(String::from("software%20development"))),
                    None,
                );
            
                let payload = AnalyticData::fetch(&api_key, query_parameters, format.clone()).unwrap();
            
                let mut response = models::TimeData {
                    category: String::from("software_development"),
                    time_data: HashMap::new(),
                };
                
                for query in payload.rows {
                    if let QueryKind::SizeSixString(value) = query {
                        if let Some(x) = response.time_data.get_mut(&value.perspective) {
                            *x += value.time_spent;
                        } else {
                            response.time_data.insert(value.perspective, value.time_spent);
                        }
                    }
                }

                time_vec.push(response);
            },
            "piano_practice" => {
                let query_parameters = Parameters::new(
                    Some(Interval),
                    Some(Day),
                    //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                    Some(Date(String::from("2019-11-05"), String::from("2020-11-04"))),
                    Some(Category),
                    Some(Thing(String::from("piano%20practice"))),
                    None,
                );
            
                let payload = AnalyticData::fetch(&api_key, query_parameters, format.clone()).unwrap();
            
                let mut response = models::TimeData {
                    category: String::from("piano_practice"),
                    time_data: HashMap::new(),
                };
                
                for query in payload.rows {
                    if let QueryKind::SizeSixString(value) = query {
                        if let Some(x) = response.time_data.get_mut(&value.perspective) {
                            *x += value.time_spent;
                        } else {
                            response.time_data.insert(value.perspective, value.time_spent);
                        }
                    }
                }

                time_vec.push(response);
            },
            _=> {
                println!("nothing!");
            }
        }
    }

    let wrapped_json = models::TimeWrapper {
        data: time_vec,
    };

    Json(wrapped_json)
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
        .mount("/", routes![get_categories, get_multi])
        .launch();

    Ok(())
}