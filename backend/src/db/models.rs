use std::collections::HashMap;
use diesel::{OptionalExtension, Queryable};
use serde::Deserialize;

use super::schema::{ date_times, skillblocks, users };

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