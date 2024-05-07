use crate::file;

pub fn missing() -> Option<String> {
    file::check().expect("failed check");

    let day = file::read().expect("failed read");

    if day.start1.is_none() {
        return Some("start1".to_owned());
    }

    if day.end1.is_none() {
        return Some("end1".to_owned());
    }

    if day.start2.is_none() {
        return Some("start2".to_owned());
    }

    if day.end2.is_none() {
        return Some("end2".to_owned());
    }

    None
}

pub fn last() -> Option<String> {
    file::check().expect("failed check");

    let day = file::read().expect("failed read");

    if day.end2.is_some() {
        return Some("end2".to_owned());
    }

    if day.start2.is_some() {
        return Some("start2".to_owned());
    }

    if day.end1.is_some() {
        return Some("end1".to_owned());
    }

    if day.start1.is_some() {
        return Some("start1".to_owned());
    }

    None
}
