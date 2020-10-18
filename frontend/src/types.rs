use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TimeSheet {
    pub notes: String,
    row_headers: Vec<String>,
    rows: Vec<String>,
}