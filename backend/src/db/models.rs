use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct TimeData {
    pub category: String,
    pub time_data: HashMap<String, u64>,
}