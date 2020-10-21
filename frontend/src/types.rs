use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticData {
    notes: String,
    row_headers: Vec<String>,
    rows: Vec<QueryKind>,

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