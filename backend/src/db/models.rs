use std::collections::HashMap;
use super::schema::skillblocks;

#[derive(Deserialize, Serialize)]
pub struct TimeData {
    pub category: String,
    pub time_data: HashMap<String, i32>,
}

// Prototype wrapper struct for storing multiple TimeData requests
#[derive(Deserialize, Serialize)]
pub struct TimeWrapper {
    pub data: Vec<TimeData>,
}

#[derive(FromForm)]
pub struct FormData {
    pub category: String,
    pub description: String,
    pub skill_name: String,
    pub username: String,
}

// Struct for querying database infromation
#[derive(Queryable, Deserialize, Serialize)]
pub struct Skillblock {
    pub id: i32,
    pub category: String,
    pub description: String,
    pub skill_name: String,
    pub username: String,
}

// Struct for database bound information
#[derive(Insertable)]
#[table_name="skillblocks"]
pub struct NewSkillblock {
    pub username: String,
    pub category: String,
    pub offline_category: bool,
    pub skill_name: String,
    pub skill_description: String,
}