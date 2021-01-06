#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use backend::auth::auth0::{ AuthParameters, Session, SessionDB, TokenResponse, build_random_state, decode_and_validate, get_or_create_user };
use backend::db::models;
use backend::db::operations::{ add_user_key, batch_add_date_times, BlockplotDbConn, create_skillblock, query_date_times_desc, query_skillblocks, update_block_count, update_date_time };

use chrono::prelude::*;
use chrono::Duration;

use dashmap::DashMap;
use models::NewDateTime;

use std::collections::HashMap;

use rocket::fairing::AdHoc;
use rocket::http::{ Cookie, Cookies, Status };
use rocket::response::{ Flash, Redirect };
use rocket::request::{ FlashMessage, Form };
use rocket::State;

use rocket_contrib::json::Json;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

use rusty_rescuetime::analytic_data::{ AnalyticData, QueryKind };
use rusty_rescuetime::parameters::Parameters;
use rusty_rescuetime::parameters::PerspectiveOptions::Interval;
use rusty_rescuetime::parameters::ResolutionOptions::Day;
use rusty_rescuetime::parameters::RestrictData::{ Date, Thing };
use rusty_rescuetime::parameters::RestrictOptions::{ Category, Overview };

use uuid::Uuid;

#[get("/")]
fn index(flash: Option<FlashMessage>) -> String {
    flash.map(|msg| format!("{}: {}", msg.name(), msg.msg()))
        .unwrap_or_else(|| "Welcome!".to_string())
}

// Route redirects to auth0 login page. Redirection link is built from
// AuthParameters instance that is managed by rocket application State
#[get("/auth0")]
fn auth0_login(mut cookies: Cookies, settings: State<AuthParameters>) -> Result<Redirect, Status> {
    let state_code = build_random_state();
    cookies.add(Cookie::new("state", state_code.clone()));

    let auth0_uri = settings.build_authorize_url(&state_code);

    Ok(Redirect::to(auth0_uri))
}

// Prototype route
#[get("/home")]
fn home(session: Session) -> Result<Json<Session>, Status> {
    Ok(Json(session))
}

// Route for testing authentication routine
#[get("/process?<code>&<state>")]
fn process_login(
    session_db: State<SessionDB>,
    code: String,
    mut cookies: Cookies,
    conn: BlockplotDbConn,
    state: String,
    settings: State<AuthParameters>
) -> Result<Redirect, Status> {
    if let Some(cookie) = cookies.get("state") {
        if state != cookie.value() {
            return Err(Status::Forbidden);
        }
    } else {
        return Err(Status::BadRequest);
    }
    cookies.remove(Cookie::named("state"));
    
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
    
    let token_payload = decode_and_validate(
        settings.client_id.as_str(),
        settings.auth0_domain.as_str(),
        token_response.id_token.as_str(),
        settings.client_secret.as_str()
    ).map_err(|_| Status::Unauthorized).unwrap();

    let user = get_or_create_user(&conn, &token_payload).map_err(|_| Status::InternalServerError)?;

    let new_session = Session {
        block_count: user.block_count, 
        email: token_payload.claims.email,
        expires: token_payload.claims.exp,
        given_name: token_payload.claims.given_name,
        key_present: user.key_present,
        nickname: token_payload.claims.nickname,
        picture: token_payload.claims.picture,
        user_id: user.auth_id,
    };

    let session_token = Uuid::new_v4().to_string();

    session_db.0.insert(session_token.to_string(), Some(new_session));

    let cookie = Cookie::build("session", session_token)
        .path("/")
        // Remember to change this to true for production
        .secure(false)
        .http_only(true)
        .finish();
    cookies.add(cookie);
        
    Ok(Redirect::to(format!("http://localhost:8080/user")))
}

// Route logs user out by retriving session_id cookie,
// uses cookie to destory session record,
// and removes session cookie.
// Redirect is then made to auth0 logout api endpoint,
// which logs user out of auth0 service and redirects
// to blockplot homepage.
#[get("/logout")]
fn process_logout(
    mut cookies: Cookies,
    session_db: State<SessionDB>,
    settings: State<AuthParameters>,
) -> Redirect {
    let session_id: Option<String> = cookies.get("session")
        .and_then(|cookie| cookie.value().parse().ok());
    if let Some(id) = session_id {
        session_db.0.remove(&id);
    }
    cookies.remove(Cookie::named("session"));

    let return_url = format!("http://localhost:8080/index");
    let logout_request = format!("https://{}/v2/logout?client_id={}&returnTo={}", settings.auth0_domain, settings.client_id, return_url);

    Redirect::to(logout_request)
}

// Route handler fetches user skillblock information from database,
// fetches timedata from RescueTime api,
// and serves processed information to frontend
#[get("/api/skillblocks")]
fn get_skillblocks(conn: BlockplotDbConn, user: models::User) -> Result<Json<models::TimeWrapper>, Status> {
    // Check user for RescueTime api key.
    // Return 404 status if not found
    if !user.key_present {
        return Err(Status::NotFound);
    }

    // TODO: Should handle the error here
    let categories = query_skillblocks(&conn, &user).unwrap();

    if categories.len() < 1 {
        return Err(Status::NotFound)
    }

    // Vector holds datastructures to be passed back to frontend
    let mut time_vec = Vec::new();

    let api_key = user.api_key.unwrap();
    let format = String::from("json");
    
    // Setup current year date
    let current_date = Local::now().date().naive_utc();
    let (current_year, current_month, current_day) = (
        current_date.year(),
        current_date.month(),
        current_date.day()
    );
    let year_end = NaiveDate::from_ymd(
        current_year,
        current_month,
        current_day
    );

    // loop through gathered database records and use information to make
    // query calls to rescuetime api for time data
    for skillblock in categories {
        let records_present = query_date_times_desc(&conn, &skillblock);
        match records_present {
            Ok(date_times) => {
                // Check if any records already exist
                if date_times.len() < 1 {
                    // Setup previous year date
                    let next_date = current_date.succ();
                    let (prev_year, prev_month, prev_day) = (
                        next_date.year() - 1,
                        next_date.month(),
                        next_date.day()
                    );
                    
                    //TODO: Currently pulling in data from less than a year. Figure out how to query data for a full year
                    let year_start = NaiveDate::from_ymd(
                        prev_year,
                        prev_month,
                        prev_day
                    );

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

                    let mut time_data_store = Vec::new();
                    for (key, val) in response.time_data.iter() {
                        let db_date_time = NewDateTime {
                            block_id: Some(skillblock.block_id),
                            day_time: *val,
                            day_date: *key,
                        };
                        time_data_store.push(db_date_time);
                    }

                    match batch_add_date_times(&conn, &time_data_store) {
                        Ok(rows) => println!("Successfully added {} rows to date database", rows),
                        Err(error) => {
                            println!("Error saving date data to db: {}", error);
                            return Err(Status::InternalServerError);
                        }
                    }
            
                    time_vec.push(response);
                } else {
                    let current_ndt = NaiveDateTime::new(
                        current_date,
                        NaiveTime::from_hms(0, 0, 0)
                    );

                    let query_parameters;

                    // Check database most recent date against
                    // current date
                    if date_times[0].0 == current_ndt {
                        if skillblock.offline_category {
                            query_parameters = Parameters::new(
                                Some(Interval),
                                Some(Day),
                                //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                                Some(Date(current_date.to_string(), current_date.to_string())),
                                Some(Category),
                                Some(Thing(skillblock.category.to_string())),
                                None,
                            );
                        } else {
                            query_parameters = Parameters::new(
                                Some(Interval),
                                Some(Day),
                                //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                                Some(Date(current_date.to_string(), current_date.to_string())),
                                Some(Overview),
                                Some(Thing(skillblock.category.to_string())),
                                None,
                            );
                        }

                        let payload = AnalyticData::fetch(&api_key, query_parameters, format.clone()).unwrap();

                        let mut data = models::TimeData {
                            category: skillblock.category,
                            skill_name: skillblock.skill_name,
                            skill_description: skillblock.description,
                            time_data: HashMap::new(),
                        };

                        // Copy database date times into hashmap
                        for dt in &date_times {
                            data.time_data.insert(dt.0, dt.1);
                        }

                        // Remove outdated current date record from hashmap
                        // TODO: Might be able to get rid of this part. Check
                        // for method that will skip insert if key already exits
                        // in hashmap
                        data.time_data.remove(&date_times[0].0);
    
                        // Recalculate time total of current date
                        // and insert updated value into hashmap
                        for query in payload.rows {
                            if let QueryKind::SizeSixString(value) = query {
                                if let Some(x) = data.time_data.get_mut(&value.perspective) {
                                    *x += value.time_spent;
                                } else {
                                    data.time_data.insert(value.perspective, value.time_spent);
                                }
                            }
                        }

                        // Grab updated date time values from hashmap
                        // and update database record
                        if let Some((key, val)) = data.time_data.get_key_value(&date_times[0].0) {
                            match update_date_time(&conn, skillblock.block_id, *key, *val) {
                                Ok(row) => println!("Successfully updated {} row!", row),
                                Err(error) => {
                                    println!("Error updating date data in db: {}", error);
                                    return Err(Status::InternalServerError);
                                }
                            }
                        }

    
                        time_vec.push(data);
                    } else {
                        // Create date older by 1 day than oldest date in database
                        let end_date = date_times[0].0.date() + Duration::days(1);
                        
                        if skillblock.offline_category {
                            query_parameters = Parameters::new(
                                Some(Interval),
                                Some(Day),
                                //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                                Some(Date(end_date.to_string(), current_date.to_string())),
                                Some(Category),
                                Some(Thing(skillblock.category.to_string())),
                                None,
                            );
                        } else {
                            query_parameters = Parameters::new(
                                Some(Interval),
                                Some(Day),
                                //TODO: Currently cloning Date's fields here. Figure out if instead lifetime identifier should be included on Parameter struct
                                Some(Date(end_date.to_string(), current_date.to_string())),
                                Some(Overview),
                                Some(Thing(skillblock.category.to_string())),
                                None,
                            );
                        }

                        let payload = AnalyticData::fetch(&api_key, query_parameters, format.clone()).unwrap();

                        let mut data = models::TimeData {
                            category: skillblock.category,
                            skill_name: skillblock.skill_name,
                            skill_description: skillblock.description,
                            time_data: HashMap::new(),
                        };
    
                        // Create hash key/values and sum total time for given category
                        for query in payload.rows {
                            if let QueryKind::SizeSixString(value) = query {
                                if let Some(x) = data.time_data.get_mut(&value.perspective) {
                                    *x += value.time_spent;
                                } else {
                                    data.time_data.insert(value.perspective, value.time_spent);
                                }
                            }
                        }

                        let mut time_data_store = Vec::new();
                        for (key, val) in data.time_data.iter() {
                            let db_date_time = NewDateTime {
                                block_id: Some(skillblock.block_id),
                                day_time: *val,
                                day_date: *key,
                            };
                            time_data_store.push(db_date_time);
                        }
                        match batch_add_date_times(&conn, &time_data_store) {
                            Ok(rows) => println!("Successfully added {} rows to date database", rows),
                            Err(error) => {
                                println!("Error saving date data to db: {}", error);
                                return Err(Status::InternalServerError);
                            }
                        }

                        for dt in date_times {
                            data.time_data.insert(dt.0, dt.1);
                        }
    
                        time_vec.push(data);
                    }
                }
            },
            Err(error) => {
                println!("Error fetching date time records: {}", error);
                return Err(Status::InternalServerError);
            }
        }
    }

    let wrapped_json = models::TimeWrapper {
        data: time_vec,
    };

    Ok(Json(wrapped_json))
}

// Prototype handler meant to handle fowards due to User RequestGuard failures
#[get("/api/skillblocks", rank = 2)]
fn get_skillblocks_redirect() -> Flash<Redirect> {
    Flash::error(Redirect::to("/"), "Invalid user login")
}

// Handle form post request and store form data into database
#[post("/api/testpost", data = "<form_data>")]
fn test_post(user: models::User, conn: BlockplotDbConn, form_data: Form<models::FormData>) -> Result<Redirect, Status> {
    if user.block_count > 3 {
        return Err(Status::Forbidden);
    }
    if !user.key_present {
        match &form_data.api_key {
            Some(key) => {
                // Consider updating db query operation to remove use of string copy 
                let query_result = add_user_key(&conn, user.auth_id.to_string(), &key);
                match query_result {
                    Ok(result) => println!("Successfully updated user key! {:?}", result),
                    Err(error) => {
                        println!("Error updating user key! {}", error);
                        return Err(Status::Forbidden);
                    }
                }
            },
            None => {
                println!("Issue with api key from form");
                return Err(Status::Forbidden);
            }
        }
    }

    let db_skillblock = models::NewSkillblock {
        user_id: Some(user.user_id),
        category: form_data.category.to_string(),
        offline_category: form_data.offline_category,
        skill_description: form_data.description.to_string(),
        skill_name: form_data.skill_name.to_string(),
    };

    create_skillblock(&conn, db_skillblock);

    // Consider updating db query operation to remove use of string copy 
    match update_block_count(&conn, user.block_count, user.auth_id.to_string()) {
        Ok(result) => println!("User block count successfully updated! {}", result),
        Err(error) => println!("Error updating user block count :( {}", error),
    }

    Ok(Redirect::to("http://localhost:8080/user"))
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
    
    let sessions = SessionDB(DashMap::new());
    
    rocket::ignite()
        .attach(BlockplotDbConn::fairing())
        .attach(cors)
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