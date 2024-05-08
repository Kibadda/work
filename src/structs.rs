#[derive(serde::Deserialize)]
pub struct Response {
    pub code: i32,
    pub result: Day,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Day {
    pub date: Option<String>,
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

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}
