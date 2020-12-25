use crate::auth::auth0::SessionDB;
use diesel::Queryable;
use rocket::request::{ FromRequest, Request, self };
use rocket::State;
use serde::Deserialize;
use std::collections::HashMap;

use super::{operations::{BlockplotDbConn, query_user}, schema::{ date_times, skillblocks, users }};

#[derive(Deserialize, Serialize)]
pub struct TimeData {
    pub username: String,
    pub category: String,
    pub skill_name: String,
    pub skill_description: String,
    pub time_data: HashMap<String, i32>,
}

// Prototype wrapper struct for storing multiple TimeData requests
#[derive(Deserialize, Serialize)]
pub struct TimeWrapper {
    pub data: Vec<TimeData>,
}

// Struct for skillblock create request
#[derive(FromForm)]
pub struct FormData {
    pub category: String,
    pub offline_category: bool,
    pub description: String,
    pub skill_name: String,
    pub username: String,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct DateTime {
    pub id: i32,
    pub block_id: Option<i32>,
    pub day_date: String,
    pub day_time: i32,
}

// Struct for querying database infromation
#[derive(Queryable, Deserialize, Serialize)]
pub struct Skillblock {
    pub block_id: i32,
    pub user_id: Option<i32>,
    pub category: String,
    pub offline_category: bool,
    pub skill_name: String,
    pub description: String,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub user_id: i32,
    pub auth_id: String,
    pub api_key: Option<String>,
    pub key_present: bool,
    pub block_count: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let pg_conn = request.guard::<BlockplotDbConn>().unwrap();
        let session_id: Option<String> = request
            .cookies()
            .get("session")
            .and_then(|cookie| cookie.value().parse().ok());

        match session_id {
            Some(id) => {
                let session_state = request.guard::<State<SessionDB>>().unwrap();
                let session_db = session_state.0.read();
                let session_map = session_db.get(&id);
                match session_map {
                    Some(key) => {
                        match *key {
                            Some(ref val) => {
                                let pg_user = query_user(&pg_conn, val.user_id.to_string());
                                match pg_user {
                                    Some(user) => {
                                        println!("{:?}", user.auth_id);
                                        return rocket::Outcome::Success(user);
                                    },
                                    None => {
                                        return rocket::Outcome::Forward(());
                                    }
                                }
                            },
                            None => {
                                println!("No user associated with session");
                                return rocket::Outcome::Forward(());
                            },
                        }
                    },
                    None => {
                        println!("Session not found in database!");
                        return rocket::Outcome::Forward(());
                    },
                }
            },
            None => println!("Session id not found!")
        }

        rocket::outcome::Outcome::Forward(())
    }
}

#[derive(Insertable)]
#[table_name="date_times"]
pub struct NewDateTime {
    block_id: Option<i32>,
    day_date: String,
    day_time: i32,
}

// Struct for database bound information
#[derive(Insertable)]
#[table_name="skillblocks"]
pub struct NewSkillblock {
    pub user_id: Option<i32>,
    pub category: String,
    pub offline_category: bool,
    pub skill_name: String,
    pub skill_description: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub auth_id: String,
    pub api_key: Option<String>,
    pub key_present: bool,
    pub block_count: i32,
}