use super::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::Serialize;
use uuid::Uuid;

// NOTE: Keep fields in order based on how comparisons should go,
// see Ord/PartialOrd Trait derive documentation
/// Struct to represent a given event on the calendar
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Serialize, Clone)]
pub struct Event {
    start: NaiveDateTime,
    end: NaiveDateTime,
    name: String,
    id: Uuid,
}

impl Event {
    /// given a start and end time determine whether they would be valid
    fn start_end_times_valid(st: &NaiveDateTime, end: &NaiveDateTime) -> bool {
        end.signed_duration_since(*st).num_seconds().is_positive()
    }

    /// return the NaiveDate component of the start field
    pub fn start(&self) -> NaiveDateTime {
        self.start
    }

    /// return the NaiveDate component of the end field
    pub fn end(&self) -> NaiveDateTime {
        self.end
    }

    /// returns the name of the event
    pub fn name(&self) -> &str {
        &self.name
    }

    /// returns the id of the event
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Create an Event with a name and date, defaults to an
    /// all day event starting at 00:00:00 and ending at 23:59:59
    pub fn new(name: String, date: &NaiveDate) -> Self {
        Self {
            name,
            start: NaiveDateTime::new(*date, day_start()),
            end: NaiveDateTime::new(*date, day_end()),
            id: Uuid::new_v4(),
        }
    }

    /// Set/Change the date and time of the start field
    pub fn set_start(self, start: NaiveDateTime) -> Result<Self, EventError> {
        // check how many seconds from the start time the end time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidStartTime error, on success returns the new start time
        if Event::start_end_times_valid(&start, &self.end) {
            // lol literally the first time ive used this syntax
            Ok(Event { start, ..self })
        } else {
            // if the new start time is invalid then return an error
            Err(EventError::InvalidStartTime)
        }
    }

    /// Set/Change an event's start time
    pub fn set_start_time(self, start: NaiveTime) -> Result<Self, EventError> {
        // check how many seconds from the start time the end time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidStartTime error, on success returns the new start time
        let new_start = NaiveDateTime::new(self.start.date(), start);
        if Event::start_end_times_valid(&new_start, &self.end) {
            // lol literally the first time ive used this syntax
            Ok(Event {
                start: new_start,
                ..self
            })
        } else {
            // if the new start time is invalid then return an error
            Err(EventError::InvalidStartTime)
        }
    }

    /// Set/Change an event's start date
    pub fn set_start_date(self, start: NaiveDate) -> Result<Self, EventError> {
        // check how many seconds from the start time the end time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidStartTime error, on success returns the new start time
        let new_start = NaiveDateTime::new(start, self.start.time());
        if Event::start_end_times_valid(&new_start, &self.end) {
            // lol literally the first time ive used this syntax
            Ok(Event {
                start: new_start,
                ..self
            })
        } else {
            // if the new start time is invalid then return an error
            Err(EventError::InvalidStartTime)
        }
    }

    /// Set/Change the date and time of the end field
    pub fn set_end(self, end: NaiveDateTime) -> Result<Self, EventError> {
        // check how many seconds from the end time the start time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidEndTime error, on success returns new end time
        if Event::start_end_times_valid(&self.start, &end) {
            // previous end time is overwritten
            Ok(Event { end, ..self })
        } else {
            Err(EventError::InvalidEndTime)
        }
    }

    /// Set/Change the time of the end field
    pub fn set_end_time(self, end: NaiveTime) -> Result<Self, EventError> {
        // check how many seconds from the end time the start time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidEndTime error, on success returns new end time
        let new_end = NaiveDateTime::new(self.end.date(), end);
        if Event::start_end_times_valid(&self.start, &new_end) {
            // previous end time is overwritten
            Ok(Event {
                end: new_end,
                ..self
            })
        } else {
            Err(EventError::InvalidEndTime)
        }
    }

    /// Set/Change the date of the end field
    pub fn set_end_date(self, end: NaiveDate) -> Result<Self, EventError> {
        // check how many seconds from the end time the start time is, if the value
        // is negative that means the start time is AFTER the end time which
        // results in an InvalidEndTime error, on success returns new end time
        let new_end = NaiveDateTime::new(end, self.end.time());
        if Event::start_end_times_valid(&self.start, &new_end) {
            // previous end time is overwritten
            Ok(Event {
                end: new_end,
                ..self
            })
        } else {
            Err(EventError::InvalidEndTime)
        }
    }

    /// Change the name of an event
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::test_fns::*;
    use super::*;
    use chrono::{Datelike, Timelike};

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
    fn test_event_serialize() {
        let nd = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        let e = Event::new("A".into(), &nd);
        let id = e.id().to_string();
        let first_time = format!("{:?}", e.start());
        let last_time = format!("{:?}", e.end());

        assert_eq!(
            e.serialize(),
            format!("{{\"start\":\"{first_time}\",\"end\":\"{last_time}\",\"name\":\"A\",\"id\":\"{id}\"}}",)
        )
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
}
