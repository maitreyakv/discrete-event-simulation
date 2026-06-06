use std::collections::BTreeMap;

use crate::{Model, event::EventKey};

pub(crate) struct EventQueue<M: Model>(BTreeMap<EventKey<M>, (M::Event, M::LogicalProcessId)>);

impl<M: Model> EventQueue<M> {
    pub(crate) fn insert(
        &mut self,
        event: M::Event,
        key: EventKey<M>,
        destination: M::LogicalProcessId,
    ) {
        self.0.insert(key, (event, destination));
    }

    pub(crate) fn pop_next(&mut self) -> Option<(EventKey<M>, M::Event, M::LogicalProcessId)> {
        self.0
            .pop_first()
            .map(|(key, (event, destination))| (key, event, destination))
    }

    pub(crate) fn remove(&mut self, key: &EventKey<M>) -> bool {
        self.0.remove(key).is_some()
    }

    pub(crate) fn time_of_next_event(&self) -> Option<&M::VirtualTime> {
        self.0
            .first_key_value()
            .map(|(key, (_event, _destination))| &key.time)
    }
}

impl<M: Model> Default for EventQueue<M> {
    fn default() -> Self {
        Self(Default::default())
    }
}
