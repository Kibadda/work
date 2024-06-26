use std::io::Write;

use crate::structs;

fn path(name: Option<String>) -> String {
    std::env::var("WORK_DIR").expect("missing dir").to_owned()
        + &match name {
            Some(name) => name,
            None => "times".to_owned(),
        }
        + ".json"
}

fn exists() -> bool {
    std::path::Path::new(&path(None)).exists()
}

fn create() -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(path(None))?;

    write!(file, "{{}}")?;

    Ok(())
}

pub fn read() -> Result<structs::Day, std::io::Error> {
    if !exists() {
        create()?;
    }

    let file = std::fs::File::open(path(None))?;

    let json: structs::Day = serde_json::from_reader(file).expect("decode failed");

    Ok(json)
}

fn move_file(date: String) -> Result<(), std::io::Error> {
    std::fs::rename(
        std::path::Path::new(&path(None)),
        std::path::Path::new(&path(Some(
            chrono::NaiveDate::parse_from_str(&date, "%d.%m.%Y")
                .expect("failed date parsing")
                .format("%Y-%m-%d")
                .to_string(),
        ))),
    )?;

    Ok(())
}

pub fn check() -> Result<(), std::io::Error> {
    let day = read()?;

    let now = chrono::Local::now();

    let date = match day.date {
        Some(date) => date,
        None => now.format("%d.%m.%Y").to_string(),
    };

    if date != now.format("%d.%m.%Y").to_string() {
        move_file(date)?;
        create()?;
    }

    Ok(())
}

pub fn write(day: &structs::Day) -> Result<(), std::io::Error> {
    if !exists() {
        create()?;
    }

    check()?;

    let mut file = std::fs::File::create(path(None))?;

    write!(
        file,
        "{}",
        serde_json::to_string(day).expect("failed json encode")
    )?;

    Ok(())
}

pub fn print() -> Result<(), std::io::Error> {
    if !exists() {
        create()?;
    }

    let day = read()?;

    println!("{day}");

    Ok(())
}
