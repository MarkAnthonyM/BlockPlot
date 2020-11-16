#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use backend::db::models;
use backend::db::operations::{ create_skillblock, query_skillblock };

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
fn get_multi(conn: BlockplotDbConn) -> Json<models::TimeWrapper> {
    dotenv().ok();
    
    let api_key = env::var("API_KEY").unwrap();
    let format = String::from("json");
    
    // Vector holds datastructures to be passed back to frontend
    let mut time_vec = Vec::new();
    let categories = query_skillblock(&conn);

    // loop through gathered database records and use information to make
    // query calls to rescuetime api for time data
    for skillblock in categories {
        let query_parameters;
        
        // Check for needed type of restrict_kind parameter
        if skillblock.offline_category {
            query_parameters = Parameters::new(
                Some(Interval),
                Some(Day),
                //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                Some(Date(String::from("2019-11-17"), String::from("2020-11-16"))),
                Some(Category),
                Some(Thing(skillblock.category.to_string())),
                None,
            );
        } else {
            query_parameters = Parameters::new(
                Some(Interval),
                Some(Day),
                //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                Some(Date(String::from("2019-11-17"), String::from("2020-11-16"))),
                Some(Overview),
                Some(Thing(skillblock.category.to_string())),
                None,
            );
        }

        let payload = AnalyticData::fetch(&api_key, query_parameters, format.clone()).unwrap();
        
        let mut response = models::TimeData {
            username: skillblock.username,
            category: skillblock.category,
            skill_name: skillblock.skill_name,
            skill_description: skillblock.description,
            time_data: HashMap::new(),
        };
        
        // Create hash key/values and sum total time for given category
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
    }

    let wrapped_json = models::TimeWrapper {
        data: time_vec,
    };

    Json(wrapped_json)
}

// Handle form post request and store form data into database
#[post("/api/testpost", data = "<form_data>")]
fn test_post(conn: BlockplotDbConn, form_data: Form<models::FormData>) -> String {
    let db_skillblock = models::NewSkillblock {
        username: form_data.username.to_string(),
        category: form_data.category.to_string(),
        offline_category: form_data.offline_category,
        skill_description: form_data.description.to_string(),
        skill_name: form_data.skill_name.to_string(),
    };

    create_skillblock(&conn, db_skillblock);

    String::from("Success!")
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
        .mount("/", routes![get_categories, get_multi, test_post])
        .launch();

    Ok(())
}