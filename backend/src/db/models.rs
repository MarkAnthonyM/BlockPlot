use chrono::naive::NaiveDateTime;
use crate::auth::auth0::SessionDB;
use diesel::Queryable;
use rocket::request::{ FromRequest, Request, self };
use rocket::State;
use serde::Deserialize;
use std::collections::HashMap;

use super::{operations::{BlockplotDbConn, query_user}, schema::{ date_times, skillblocks, users }};

#[derive(Deserialize, Serialize)]
pub struct TimeData {
    pub category: String,
    pub skill_name: String,
    pub skill_description: String,
    pub time_data: HashMap<NaiveDateTime, i32>,
}

// Prototype wrapper struct for storing multiple TimeData requests
#[derive(Deserialize, Serialize)]
pub struct TimeWrapper {
    pub data: Vec<TimeData>,
}

// Struct for skillblock create request
#[derive(FromForm)]
pub struct FormData {
    pub api_key: Option<String>,
    pub category: String,
    pub offline_category: bool,
    pub description: String,
    pub skill_name: String,
}

#[derive(Associations, Identifiable, Queryable, Deserialize, Serialize)]
#[belongs_to(Skillblock, foreign_key = "block_id")]
pub struct DateTime {
    pub id: i32,
    pub block_id: Option<i32>,
    pub day_time: i32,
    pub day_date: NaiveDateTime,
}

// Struct for querying database infromation
#[derive(Associations, Identifiable, Queryable, Deserialize, Serialize)]
#[primary_key(block_id)]
#[belongs_to(User, foreign_key = "user_id")]
pub struct Skillblock {
    pub block_id: i32,
    pub user_id: Option<i32>,
    pub category: String,
    pub offline_category: bool,
    pub skill_name: String,
    pub description: String,
}

// Struct for querying user information from postgres database
// Is also a request guard for various endpoints
#[derive(Identifiable, Queryable, Deserialize, Serialize)]
#[primary_key(user_id)]
pub struct User {
    pub user_id: i32,
    pub auth_id: String,
    pub api_key: Option<String>,
    pub key_present: bool,
    pub block_count: i32,
}

// Requst guard implementation. Validation policy will
// check for session, determine if session is associated with a logged user
// and verify if session is still valid.
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        // Connection pool used to interact with postgres database.
        let pg_conn = request.guard::<BlockplotDbConn>().unwrap();
        let session_id: Option<String> = request
            .cookies()
            .get("session")
            .and_then(|cookie| cookie.value().parse().ok());
        
        match session_id {
            Some(id) => {
                // Grab in memory sessions owning database. Use session id retrived from
                // cookies to query for a valid session
                let session_db = request.guard::<State<SessionDB>>().unwrap().inner();
                let session_map = session_db.0.get(&id).unwrap();
                // Check for Session struct associated with session key.
                // If none found, forward to login endpoint
                match *session_map {
                    Some(ref session) => {
                        if session.session_expired() {
                            return rocket::Outcome::Forward(());
                        }

                        // Query postgres database for user. If match found,
                        // return Success outcome, passing retrived User to
                        // calling endpoint
                        let pg_user = query_user(&pg_conn, session.user_id.to_string());
                        match pg_user {
                            Some(user) => {
                                rocket::Outcome::Success(user)
                            },
                            None => {
                                rocket::Outcome::Forward(())
                            }
                        }
                    },
                    None => {
                        rocket::Outcome::Forward(())
                    }
                }
            },
            None => {
                rocket::Outcome::Forward(())
            }
        }
    }
}

// Struct for creating and inserting a new record
// for a given date and time returned from
// RescueTime Api
#[derive(Insertable)]
#[table_name="date_times"]
pub struct NewDateTime {
    pub block_id: Option<i32>,
    pub day_time: i32,
    pub day_date: NaiveDateTime,
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

// Struct for creating new user record
// for database insertion
#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub auth_id: String,
    pub api_key: Option<String>,
    pub key_present: bool,
    pub block_count: i32,
}