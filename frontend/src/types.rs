use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Struct represents different color options for heatmap shading
//TODO: Explore enum equivalent
#[non_exhaustive]
pub struct Color;

impl Color {
    pub const NEUTRAL: &'static str = "#dadada";
    pub const LIGHT: &'static str = "#dac695";
    pub const LIGHTMEDIUM: &'static str = "#f28a00";
    pub const MEDIUM: &'static str = "#fd4600";
    pub const MEDIUMHIGH: &'static str = "#f1230b";
    pub const HIGH: &'static str = "#bc1c2a";
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub block_count: i32,
    pub email: String,
    pub expires: i64,
    pub given_name: String,
    pub key_present: bool,
    pub nickname: String,
    pub picture: String,
    pub user_id: String,
}

// Incoming timedata payload deserializes to this struct
#[derive(Deserialize, Serialize)]
pub struct TimeData {
    pub category: String,
    pub skill_name: String,
    pub skill_description: String,
    pub time_data: HashMap<NaiveDateTime, i32>,
}

// Store various stat calculations from user time data
pub struct TimeStats {
    pub daily_max: i32,
    pub yearly_max: i32,
    pub longest_chain: i32,
}

// Prototype struct being used to test handling request with multiple TimeData response objects. May remove at the conclusion of tests
#[derive(Deserialize, Serialize)]
pub struct TimeWrapper {
    pub data: Vec<TimeData>,
}
