use crate::{cli, file, slot, structs};

fn request(endpoint: &str, overrides: Option<[structs::Data; 1]>) -> Result<(), structs::Error> {
    let response = reqwest::blocking::Client::new()
        .post(std::env::var("WORK_URL").expect("missing url").to_owned() + endpoint)
        .json(&structs::Payload {
            token: std::env::var("WORK_TOKEN").expect("missing token"),
            overrides,
        })
        .send()?;

    let json: structs::Response = response.json()?;

    if json.code != 200 {
        return Err(structs::Error {
            message: format!("response code: {}", json.code),
        });
    }

    file::write(&json.result)?;
    file::print()?;

    Ok(())
}

pub fn show() -> Result<(), structs::Error> {
    request("today", None)
}

pub fn track(time: &cli::Time) -> Result<(), structs::Error> {
    request(
        "enter",
        Some([structs::Data {
            key: slot::missing().ok_or(structs::Error {
                message: "all slots are already set. use update instead".to_string(),
            })?,
            value: time.to_owned().value,
        }]),
    )
}

pub fn update(time: &cli::Time, slot: &cli::Slot) -> Result<(), structs::Error> {
    let slot = slot.to_owned().value.ok_or(structs::Error {
        message: "invalid slot".to_string(),
    })?;

    request(
        "enter",
        Some([structs::Data {
            key: match slot.is_empty() {
                false => slot,
                true => {
                    return Err(structs::Error {
                        message: "no slots are set. use track instead".to_string(),
                    })
                }
            },
            value: time.to_owned().value,
        }]),
    )
}
