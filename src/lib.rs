use thiserror::Error;

mod cal;
mod event;

pub use cal::EventCalendar;
pub use event::Event;
use uuid::Uuid;

pub trait IntoUuid {
    fn into_uuid(self) -> Uuid;
}

impl IntoUuid for &str {
    fn into_uuid(self) -> Uuid {
        use std::str::FromStr;
        Uuid::from_str(self).unwrap() // this is bad and will need to be fixed
    }
}

impl IntoUuid for &Uuid {
    fn into_uuid(self) -> Uuid {
        *self
    }
}

impl IntoUuid for Uuid {
    fn into_uuid(self) -> Uuid {
        self
    }
}

/// Basic Errors that can occur for events
#[derive(Error, Debug)]
pub enum EventError {
    /// Error for invalid start time for an event
    #[error("start time/date cannot be after end time/date")]
    InvalidStartTime,

    /// Error for invalid end time for an event
    #[error("end time/date cannot be before start time/date")]
    InvalidEndTime,
}

/// returns a NaiveTime of 11:59:59
///
/// # Examples
/// ```
/// use calib::day_end;
/// use chrono::NaiveTime;
///
/// let last_time_of_day = day_end();
/// assert_eq!(last_time_of_day, NaiveTime::from_hms_opt(23, 59, 59).unwrap())
/// ```
pub fn day_end() -> chrono::NaiveTime {
    chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap()
}

/// returns a NaiveTime of 00:00:00
/// /// # Examples
/// ```
/// use calib::day_start;
/// use chrono::NaiveTime;
///
/// let first_time_of_day = day_start();
/// assert_eq!(first_time_of_day, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
/// ```
pub fn day_start() -> chrono::NaiveTime {
    chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
}

#[cfg(test)]
mod test {
    use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

    use super::*;

    // helper functions for test
    /// return the first NaiveDate for 2023
    fn first_day_2023_nd() -> NaiveDate {
        NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
    }

    /// return the first time of any day 00:00:00
    fn first_time_nt() -> NaiveTime {
        NaiveTime::from_hms_opt(0, 0, 0).unwrap()
    }

    /// return the last time for any day 23:59:59
    fn last_time_nt() -> NaiveTime {
        NaiveTime::from_hms_opt(23, 59, 59).unwrap()
    }

    /// return the first NaiveDateTime for 2023 - 01/01/2023-00:00:00
    fn first_day_2023_ndt() -> NaiveDateTime {
        let nd = first_day_2023_nd();
        let nt = first_time_nt();
        NaiveDateTime::new(nd, nt)
    }

    // ##################################
    // ###           TESTS            ###
    // ##################################

    #[test]
    fn test_new_event() {
        let naive_date = first_day_2023_nd();

        // common times
        let first_time = first_time_nt();
        let last_time = last_time_nt();

        // event being tested
        let event = Event::new(String::from("Birthday Party"), &naive_date);

        // assumed start and end times for testing
        let assumed_start_time = NaiveDateTime::new(naive_date, first_time);
        let assumed_end_time = NaiveDateTime::new(naive_date, last_time);

        assert_eq!(event.start(), assumed_start_time);
        assert_eq!(event.end(), assumed_end_time);
    }

    #[test]
    fn test_event_start_time_change() {
        // basic date declaration
        let naive_date = first_day_2023_nd();

        // event being tested
        let mut event = Event::new(String::from("Birthday Party"), &naive_date);
        // new start time
        let new_start_time = NaiveTime::from_hms_opt(10, 30, 0).unwrap();

        event = event
            .set_start(NaiveDateTime::new(naive_date, new_start_time))
            .unwrap();
        assert_eq!(
            event.start(),
            NaiveDateTime::new(naive_date, new_start_time)
        )
    }

    #[test]
    fn test_event_end_time_change() {
        // basic date declaration
        let naive_date = first_day_2023_nd();

        // event being tested
        let mut event = Event::new(String::from("Birthday Party"), &naive_date);
        // new start time
        let new_end_time = NaiveTime::from_hms_opt(22, 30, 0).unwrap();

        event = event
            .set_end(NaiveDateTime::new(naive_date, new_end_time))
            .unwrap();

        assert_eq!(event.end(), NaiveDateTime::new(naive_date, new_end_time))
    }

    #[test]
    fn test_invalid_event_time_change() {
        // basic date declaration
        let naive_date = first_day_2023_nd();
        let start_time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let invalid_end_time = NaiveTime::from_hms_opt(10, 0, 0).unwrap();

        let mut event = Event::new("Birthday".into(), &naive_date);

        event = event
            .set_start(NaiveDateTime::new(naive_date, start_time))
            .unwrap();

        assert_eq!(
            true,
            event
                .set_end(NaiveDateTime::new(naive_date, invalid_end_time))
                .is_err()
        );
    }

    #[test]
    fn invalid_events_test() {
        // basic date declaration
        let naive_date = first_day_2023_nd();

        // common times
        let first_time = first_time_nt();
        let last_time = last_time_nt();

        // event being tested
        let mut event = Event::new(String::from("Birthday Party"), &naive_date);

        // assumed start and end times for testing
        let assumed_start_time = NaiveDateTime::new(naive_date, first_time);
        let assumed_end_time = NaiveDateTime::new(naive_date, last_time);

        assert_eq!(event.start(), assumed_start_time);
        assert_eq!(event.end(), assumed_end_time);

        // new start time
        let new_start_time = NaiveTime::from_hms_opt(10, 30, 0).unwrap();

        // update start time
        event = event
            .set_start(NaiveDateTime::new(naive_date, new_start_time))
            .unwrap();

        assert_eq!(
            event.start(),
            NaiveDateTime::new(naive_date, new_start_time)
        );

        // new end time
        let new_end_time = NaiveTime::from_hms_opt(22, 30, 0).unwrap();

        // update end time
        event = event
            .set_end(NaiveDateTime::new(naive_date, new_end_time))
            .unwrap();

        assert_eq!(event.end(), NaiveDateTime::new(naive_date, new_end_time));

        // try to set invalid start time
        let status = event.set_start(NaiveDateTime::new(naive_date, last_time));
        assert_eq!(true, status.is_err());

        // try to set invalid end time
        let event = Event::new(String::from("Birthday Party"), &naive_date);
        let status = event.set_end(NaiveDateTime::new(naive_date, first_time));
        assert_eq!(true, status.is_err());
    }

    #[test]
    fn test_event_ordering_lt_start_cmp() {
        use std::cmp::Ordering;
        let ndt = first_day_2023_ndt();
        let d1 = Event::new("A".into(), &ndt.date());

        // 01/01/2023-00:00:00 < 01/01/2023-00:00:01
        let mut d2 = Event::new("A".into(), &ndt.date());
        d2 = d2.set_start(d1.start().with_second(1).unwrap()).unwrap();
        assert_eq!(d1.cmp(&d2), Ordering::Less);

        // 01/01/2023-00:00:00 < 01/01/2023-00:01:00
        let mut d3 = Event::new("A".into(), &ndt.date());
        d3 = d3.set_start(d1.start().with_minute(1).unwrap()).unwrap();
        assert_eq!(d1.cmp(&d3), Ordering::Less);

        // 01/01/2023-00:00:00 < 01/01/2023-01:00:00
        let mut d4 = Event::new("A".into(), &ndt.date());
        d4 = d4.set_start(d1.start().with_hour(1).unwrap()).unwrap();
        assert_eq!(d1.cmp(&d4), Ordering::Less);

        // 01/01/2023-00:00:00 < 01/01/2024-00:00:00
        let d5 = Event::new("A".into(), &ndt.date().with_year(2024).unwrap());
        assert_eq!(d1.cmp(&d5), Ordering::Less);

        // 01/01/2023-00:00:00 < 01/02/2023-00:00:00
        let mut d6 = Event::new("A".into(), &ndt.date());
        d6 = d6.set_end(d1.start().with_day(3).unwrap()).unwrap();
        d6 = d6.set_start(d1.start().with_day(2).unwrap()).unwrap();
        assert_eq!(d1.cmp(&d6), Ordering::Less);

        // 01/01/2023-00:00:00 < 02/01/2023-00:00:00
        let mut d7 = Event::new("A".into(), &ndt.date());
        d7 = d7.set_end(d1.start().with_month(3).unwrap()).unwrap();
        d7 = d7.set_start(d1.start().with_month(2).unwrap()).unwrap();
        assert_eq!(d1.cmp(&d7), Ordering::Less);
    }

    #[test]
    fn test_event_range() {
        let nd1 = first_day_2023_nd();
        let nd2 = nd1.with_day(2).unwrap();
        let nd3 = nd1.with_day(3).unwrap();
        let nd4 = nd1.with_day(4).unwrap();
        let nd5 = nd1.with_day(5).unwrap();

        let e1 = Event::new("A".into(), &nd1);

        let e2 = Event::new("B".into(), &nd2);
        let e2_id = *e2.id();

        let e3 = Event::new("C".into(), &nd3);
        let e3_id = *e3.id();

        let e4 = Event::new("D".into(), &nd4);
        let e4_id = *e4.id();

        let e5 = Event::new("E".into(), &nd5);

        // range is from 01/02/2023T11:00:00 to 01/04/2023T23:59:59
        let range_start = NaiveDateTime::new(nd2, NaiveTime::from_hms_opt(11, 0, 0).unwrap());
        let range_end = NaiveDateTime::new(nd4, day_end());

        let mut cal = EventCalendar::default();

        cal.add_event(e1);
        cal.add_event(e2);
        cal.add_event(e3);
        cal.add_event(e4);
        cal.add_event(e5);

        let mut iter = cal.events_in_range(range_start, range_end);

        assert_eq!(iter.next(), cal.get(&e2_id));
        assert_eq!(iter.next(), cal.get(&e3_id));
        assert_eq!(iter.next(), cal.get(&e4_id));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_event_serialize() {
        let nd = first_day_2023_nd();
        let e = Event::new("A".into(), &nd);
        let id = e.id().to_string();

        let first_time = first_day_2023_ndt().format("%Y-%m-%dT%H:%M:%S").to_string();
        let last_time = NaiveDateTime::new(nd, last_time_nt())
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string();

        assert_eq!(
            e.serialize(),
            format!("{{\"start\":\"{first_time}\",\"end\":\"{last_time}\",\"name\":\"A\",\"id\":\"{id}\"}}",)
        )
    }
}
