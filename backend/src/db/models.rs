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