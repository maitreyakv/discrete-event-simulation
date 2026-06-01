use std::collections::BTreeMap;

use crate::{Model, event::EventKey};

pub(crate) struct EventQueue<M: Model>(BTreeMap<EventKey<M>, (M::Event, M::LogicalProcessId)>);

impl<M: Model> EventQueue<M> {
    pub(crate) fn insert(
        &mut self,
        event: M::Event,
        event_key: EventKey<M>,
        destination: M::LogicalProcessId,
    ) {
        self.0.insert(event_key, (event, destination));
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
