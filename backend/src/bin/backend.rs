#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use backend::auth::auth0::{ AuthParameters, TokenResponse, UserInfo };
use backend::db::models;
use backend::db::operations::{ create_skillblock, query_skillblock };

use chrono::prelude::*;

use dotenv::dotenv;

use serde_json::ser::to_vec;

use std::collections::HashMap;
use std::env;

use rocket::fairing::AdHoc;
use rocket::http::{ Cookies, Status };
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::State;

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

// Route redirects to auth0 login page. Redirection link is built from
// AuthParameters instance that is managed by rocket application State
#[get("/auth0")]
fn auth0_login(settings: State<AuthParameters>) -> Result<Redirect, Status> {
    let auth0_uri = settings.build_authorize_url();

    Ok(Redirect::to(auth0_uri))
}

// Route for testing authentication routine
#[get("/process?<code>&<state>")]
fn process_login(
    code: String,
    mut cookies: Cookies,
    state: String,
    settings: State<AuthParameters>
) -> Result<Redirect, Status> {
    let token_parameters = settings.build_token_request(&code);
    let serialized = serde_json::to_string(&token_parameters).unwrap();
    let token_url = format!("https://{}/oauth/token", settings.auth0_domain);

    let client = reqwest::blocking::Client::new();
    let token_response: TokenResponse = client
        .post(&token_url)
        .header("content-type", "application/json")
        .body(serialized)
        .send()
        .unwrap()
        .json()
        .expect("Error with token request");

    let user_info = format!("https://{}/userinfo", settings.auth0_domain);
    let token_key = format!("Bearer {}", token_response.access_token);
    let response: UserInfo = client
        .get(&user_info)
        .header("Authorization", token_key)
        .send()
        .unwrap()
        .json()
        .expect("Error with user info response");
        
    Ok(Redirect::to("/about"))
}

// Route handler fetches user skillblock information from database,
// fetches timedata from RescueTime api,
// and serves processed information to frontend
#[get("/api/skillblocks")]
fn get_skillblocks(conn: BlockplotDbConn) -> Json<models::TimeWrapper> {
    dotenv().ok();
    
    let api_key = env::var("API_KEY").unwrap();
    let format = String::from("json");
    
    // Setup dates
    let current_date = Local::now().date().naive_utc();
    let (current_year, current_month, current_day) = (
        current_date.year(),
        current_date.month(),
        current_date.day()
    );

    //TODO: Currently pulling in data from less than a year. Figure out how to query data for a full year
    let year_start = NaiveDate::from_ymd(
        current_year - 1,
        current_month,
        current_day + 1
    );
    let year_end = NaiveDate::from_ymd(
        current_year,
        current_month,
        current_day
    );
    
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
                Some(Date(year_start.to_string(), year_end.to_string())),
                Some(Category),
                Some(Thing(skillblock.category.to_string())),
                None,
            );
        } else {
            query_parameters = Parameters::new(
                Some(Interval),
                Some(Day),
                //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                Some(Date(year_start.to_string(), year_end.to_string())),
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
        .mount("/", routes![auth0_login, get_skillblocks, process_login, test_post])
        .attach(AdHoc::on_attach("Parameters Config", |rocket| {
            let config = rocket.config();
            let auth_parameters = AuthParameters::new(config).unwrap();

            Ok(rocket.manage(auth_parameters))
        }))
        .launch();

    Ok(())
}