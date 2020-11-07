use chrono::NaiveDateTime;
use std::collections::HashMap;
use serde::{ Deserialize, Serialize };

//TODO: Currently declaring structs for json data deserialization twice.
// Should used data types from Rusty-RescueTime when possible  
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticData {
    pub notes: String,
    pub row_headers: Vec<String>,
    pub rows: Vec<QueryKind>,

}

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

//TODO: Current method of deserialization feels too messy. Try to find A more concise way to work with json data
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum QueryKind {
    SizeFourInt(SizeFour<i32, i32>),
    SizeFourMixedInt(SizeFour<i32, String>),
    SizeFourMixedString(SizeFour<String, i32>),
    SizeFourString(SizeFour<String, String>),
    SizeSevenInt(SizeSeven<i32>),
    SizeSevenString(SizeSeven<String>),
    SizeSixInt(SizeSix<i32>),
    SizeSixString(SizeSix<String>),
}

// Struct represents individual cell data related to the row_headers field of the AnalyticData struct.
// May not need this struct
#[derive(Debug, Deserialize, Serialize)]
pub struct SizeFour<T, U> {
    pub perspective: T,
    pub time_spent: i32,
    pub number_of_people: i32,
    pub restrict_kind: U,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SizeSeven<T> {
    pub perspective: T,
    pub time_spent: i32,
    pub number_of_people: i32,
    pub activity: String,
    pub document: String,
    pub category: String,
    pub productivity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SizeSix<T> {
    pub perspective: T,
    pub time_spent: i32,
    pub number_of_people: i32,
    pub activity: String,
    pub category: String,
    pub productivity: i32,
}

// Contains information about time data, information related to time data for a given category,
// and color information used to fill blocks  
pub struct SkillBlock {
    pub category: String,
    pub description: String,
    pub name: String,
    pub recent_time_data: TimeData,
    pub block_color_lite: String,
    pub block_color_regular: String,
    pub block_color_deep: String,
}

//TODO: Find way to deserialize with Chrono datetime type
#[derive(Deserialize, Serialize)]
pub struct TimeData {
    pub category: String,
    pub time_data: HashMap<NaiveDateTime, i32>,
}

// Prototype struct being used to test handling request with multiple TimeData response objects. May remove at the conclusion of tests
#[derive(Deserialize, Serialize)]
pub struct TimeWrapper {
    pub data: Vec<TimeData>,
}