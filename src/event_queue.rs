use std::collections::{BTreeMap, btree_map::Entry};

use crate::{Model, event::EventKey};

pub(crate) struct EventQueue<M: Model>(BTreeMap<EventKey<M>, (M::Event, M::LogicalProcessId)>);

impl<M: Model> EventQueue<M> {
    pub(crate) fn try_insert(
        &mut self,
        event: M::Event,
        event_key: EventKey<M>,
        destination: M::LogicalProcessId,
    ) -> Result<(), DuplicateEventError> {
        match self.0.entry(event_key) {
            Entry::Vacant(v) => {
                v.insert((event, destination));
                Ok(())
            }
            Entry::Occupied(_) => Err(DuplicateEventError),
        }
    }

    pub(crate) fn pop_next(&mut self) -> Option<(EventKey<M>, M::Event, M::LogicalProcessId)> {
        self.0
            .pop_first()
            .map(|(event_key, (event, destination))| (event_key, event, destination))
    }
}

impl<M: Model> Default for EventQueue<M> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Duplicate event detected")]
pub(crate) struct DuplicateEventError;
