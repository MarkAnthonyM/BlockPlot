use std::collections::HashMap;

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

#[derive(Clone, Deserialize, Serialize)]
pub struct FormData {
    pub category: String,
    pub description: String,
    pub skill_name: String,
}

// Struct for querying database infromation
#[derive(Queryable, Deserialize, Serialize)]
pub struct Skillblock {
    pub id: i32,
    pub category: String,
    pub description: String,
    pub skill_name: String,
}