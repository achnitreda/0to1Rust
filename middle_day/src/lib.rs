use chrono::{Weekday, NaiveDate,Datelike};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    // year % 400 == 0
    // year % 4 == 0 && year % 100 != 0
    match is_leap_year(year) {
        Some(y) => {
            NaiveDate::from_yo_opt(y as i32, 183).map(|date| date.weekday())
        },
        None => None
    }
}

fn is_leap_year(year : u32) -> Option<u32> {
    if year % 400 == 0 {
        return None
    }else if year % 4 == 0 && year % 100 != 0 {
        return None
    }else {
        Some(year)
    }
}
