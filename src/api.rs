use serde::{Deserialize, Serialize};
use std::env::var;

static BASE_URL: &str = "https://timekeeping-reloaded.in.cortex-media.de/api/";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Day {
    date: String,
    start1: String,
    end1: String,
    start2: String,
    end2: String,
}

#[derive(Serialize)]
struct Payload {
    token: String,
    overrides: Option<[Data; 1]>,
}

#[derive(Serialize)]
struct Data {
    key: String,
    value: String,
}

fn request(endpoint: &str, data: Option<Data>) -> Day {
    let token = var("WORK_TOKEN").expect("missing token");

    let payload = Payload {
        token,
        overrides: data.map(|data| [data]),
    };

    let url = BASE_URL.to_owned() + endpoint;

    let client = reqwest::blocking::Client::new();
    let response = client.post(url).json(&payload).send().unwrap();

    response.json().unwrap()
}

pub fn today() -> Day {
    request("today", None)
}

pub fn enter(identifier: String, time: Option<String>) -> Day {
    request(
        "enter",
        Some(Data {
            key: identifier,
            value: match time {
                Some(time) => time,
                None => "now".to_owned(),
            },
        }),
    )
}
