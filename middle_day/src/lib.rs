
use chrono::{Datelike, NaiveDate};
pub use chrono::Weekday as wd;

pub fn middle_day(year: i32) -> Option<wd> {
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    let days_in_year = if is_leap_year { 366 } else { 365 };
    if days_in_year % 2 == 0 {
        return None;
    }
    let middle_day = NaiveDate::from_ymd_opt(year, 1, 1)?.num_days_from_ce() + days_in_year / 2;
    NaiveDate::from_num_days_from_ce_opt(middle_day as i32).map(|date| date.weekday())
}