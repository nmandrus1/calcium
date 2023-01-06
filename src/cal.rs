use chrono::NaiveDateTime;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

use super::{event::Event, IntoUuid};

// Maybe use a BTreeSet to keep events in chronological order
// and then add a second field which is a Hashmap<UUID, &Event>
// keep the BTreeSet as append-only and only edit events through
// dereferencing hashmap

/// Represents a calendar of events
#[derive(Default)]
pub struct EventCalendar {
    ids: BTreeMap<Uuid, Event>,
    evts: BTreeSet<Event>,
}

impl EventCalendar {
    /// inserts event into calednar, returning true if the event
    /// is new to the calendar and false if the event already exits
    pub fn add_event(&mut self, event: Event) -> bool {
        self.ids.insert(*event.id(), event.clone());
        self.evts.insert(event)
    }

    /// return an iterator of all events between start and end
    pub fn events_in_range(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> impl Iterator<Item = &Event> {
        self.evts.iter().filter(move |evt| {
            (evt.start() >= start && evt.start() <= end) || (evt.end() >= start && evt.end() <= end)
        })
    }

    /// return the first event in the Calendar
    pub fn first_event(&self) -> Option<&Event> {
        self.evts.first()
    }

    /// return a reference to an event from it's ID
    pub fn get<T: IntoUuid>(&self, id: T) -> Option<&Event> {
        self.ids.get(&id.into_uuid())
    }

    /// remove an event from the calendar
    pub fn remove<T: IntoUuid>(&mut self, id: T) -> Option<Event> {
        let evt = self.ids.remove(&id.into_uuid())?;
        match self.evts.remove(&evt) {
            true => Some(evt),
            false => None,
        }
    }
}
