use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct TimeData {
    category: String,
    time_data: HashMap<String, u64>,
}