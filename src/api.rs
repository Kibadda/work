use crate::{cli, file, slot, structs};

static BASE_URL: &str = "http://timekeeping-reloaded.in.cortex-media.de/api/";

fn request(endpoint: &str, overrides: Option<[structs::Data; 1]>) -> Result<(), String> {
    match reqwest::blocking::Client::new()
        .post(BASE_URL.to_owned() + endpoint)
        .json(&structs::Payload {
            token: std::env::var("WORK_TOKEN").expect("missing token"),
            overrides,
        })
        .send()
    {
        Ok(response) => match response.json::<structs::Response>() {
            Ok(json) => match json.code {
                200 => {
                    if let Err(err) = file::write(&json.result) {
                        return Err(err.to_string());
                    }
                    if let Err(err) = file::print() {
                        return Err(err.to_string());
                    }

                    Ok(())
                }
                _ => Err(format!("reponse code: {}", json.code)),
            },
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}

pub fn show() -> Result<(), String> {
    request("today", None)
}

pub fn track(time: &cli::Time) -> Result<(), String> {
    request(
        "enter",
        Some([structs::Data {
            key: match slot::missing() {
                Some(slot) => slot,
                None => return Err("all slots are already set. use update instead".to_string()),
            },
            value: time.to_owned().value,
        }]),
    )
}

pub fn update(time: &cli::Time, slot: &cli::Slot) -> Result<(), String> {
    request(
        "enter",
        Some([structs::Data {
            key: match slot.to_owned().value {
                Some(slot) => {
                    if slot.is_empty() {
                        return Err("no slots are set. use track instead".to_string());
                    }

                    slot
                }
                None => {
                    return Err("invalid slot".to_string());
                }
            },
            value: time.to_owned().value,
        }]),
    )
}
