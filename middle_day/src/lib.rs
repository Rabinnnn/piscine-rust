use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    // Check if it's a leap year (has 366 days), otherwise return None
    let is_leap_year = |y: i32| -> bool {
        (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
    };

    if !is_leap_year(year) {
        // Non-leap years have 365 days — middle day exists
        let middle_day = NaiveDate::from_ymd_opt(year, 1, 1)?.succ().checked_add_days(chrono::Days::new(182))?;
        Some(middle_day.weekday())
    } else {
        // Leap years have 366 days — no middle day
        None
    }
}
