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

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use colored::Colorize;
        write!(
            f,
            "Datum: {}\n{} - {}\n{} - {}",
            match &self.date {
                Some(date) => date.green(),
                None => "n/a".red(),
            },
            match &self.start1 {
                Some(start1) => start1.blue(),
                None => "n/a".red(),
            },
            match &self.end1 {
                Some(end1) => end1.blue(),
                None => "n/a".red(),
            },
            match &self.start2 {
                Some(start2) => start2.blue(),
                None => "n/a".red(),
            },
            match &self.end2 {
                Some(end2) => end2.blue(),
                None => "n/a".red(),
            },
        )
    }
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
