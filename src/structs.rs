#[derive(serde::Deserialize)]
pub struct Response {
    pub code: i32,
    pub result: Day,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Day {
    pub date: String,
    pub start1: Option<String>,
    pub end1: Option<String>,
    pub start2: Option<String>,
    pub end2: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Payload {
    pub token: String,
    pub overrides: Option<[Data; 1]>,
}

#[derive(serde::Serialize)]
pub struct Data {
    pub key: String,
    pub value: String,
}
