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
        use chrono::{Local, NaiveTime};
        use colored::Colorize;

        fn parse_time(s: &Option<String>) -> Option<NaiveTime> {
            s.as_ref()
                .and_then(|t| NaiveTime::parse_from_str(t, "%H:%M").ok())
        }

        let now = Local::now().time();

        let start1 = parse_time(&self.start1);
        let end1 =
            parse_time(&self.end1).or_else(|| if start1.is_some() { Some(now) } else { None });

        let start2 = parse_time(&self.start2);
        let end2 =
            parse_time(&self.end2).or_else(|| if start2.is_some() { Some(now) } else { None });

        let mut total_secs = 0i64;

        if let (Some(s), Some(e)) = (start1, end1) {
            total_secs += (e - s).num_seconds();
        }

        if let (Some(s), Some(e)) = (start2, end2) {
            total_secs += (e - s).num_seconds();
        }

        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;

        let time = format!("{:02}:{:02}", hours, minutes);

        write!(
            f,
            "Datum: {}\n{} - {}\n{} - {}\nZeit: {}",
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
            time.yellow(),
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
