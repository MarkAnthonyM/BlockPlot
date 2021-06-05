use crate::db::operations::{add_user_key, batch_add_date_times, BlockplotDbConn, create_skillblock, query_date_times_desc, query_skillblocks, update_block_count, update_date_time};
use crate::db::models;
use crate::db::models::NewDateTime;

use chrono::prelude::*;
use chrono::Duration;

use rocket::http::Status;
use rocket::request::Form;
use rocket::response::{ Flash, Redirect };
use rocket_contrib::json::Json;

use rusty_rescuetime::analytic_data::{ AnalyticData, QueryKind };
use rusty_rescuetime::parameters::Parameters;
use rusty_rescuetime::parameters::PerspectiveOptions::Interval;
use rusty_rescuetime::parameters::ResolutionOptions::Day;
use rusty_rescuetime::parameters::RestrictData::{ Date, Thing };
use rusty_rescuetime::parameters::RestrictOptions::{ Category, Overview };

use std::collections::HashMap;

// Route handler fetches user skillblock information from database,
// fetches timedata from RescueTime api,
// and serves processed information to frontend
#[get("/api/skillblocks")]
fn get_skillblocks(conn: BlockplotDbConn, user: models::User) -> Result<Json<models::TimeWrapper>, Status> {
    // Check user for RescueTime api key.
    // Return 404 status if not found
    //TODO: Return more appropriate status code here
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