use std::collections::HashMap;
use super::schema::{ skillblocks, users };

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
    pub id: i32,
    pub username: String,
    pub category: String,
    pub offline_category: bool,
    pub skill_name: String,
    pub description: String,
}

// Struct for database bound information
#[derive(Insertable)]
#[table_name="skillblocks"]
pub struct NewSkillblock {
    pub block_id: i32,
    pub user_id: Option<i32>,
    pub category: String,
    pub offline_category: bool,
    pub skill_name: String,
    pub skill_description: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub user_id: i32,
    pub auth_id: String,
    pub api_key: Option<String>,
    pub key_present: bool,
    pub block_count: i32,
}