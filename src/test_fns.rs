use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

// helper functions for test
/// return the first NaiveDate for 2023
pub fn first_day_2023_nd() -> NaiveDate {
    NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
}

/// return the first time of any day 00:00:00
pub fn first_time_nt() -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
}

/// return the last time for any day 23:59:59
pub fn last_time_nt() -> NaiveTime {
    NaiveTime::from_hms_opt(23, 59, 59).unwrap()
}

/// return the first NaiveDateTime for 2023 - 01/01/2023-00:00:00
pub fn first_day_2023_ndt() -> NaiveDateTime {
    let nd = first_day_2023_nd();
    let nt = first_time_nt();
    NaiveDateTime::new(nd, nt)
}
