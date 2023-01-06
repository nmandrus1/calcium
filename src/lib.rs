use thiserror::Error;

mod cal;
mod event;

// functions to make setting up tests much easier
#[cfg(test)]
mod test_fns;

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
    use chrono::{Datelike, NaiveDateTime, NaiveTime};

    use super::test_fns::*;
    use super::*;

    // ##################################
    // ###           TESTS            ###
    // ##################################

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
}
