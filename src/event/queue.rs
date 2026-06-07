use std::collections::BTreeMap;

use crate::Model;
use crate::event::AntiEvent;

use super::Event;
use super::EventKey;

#[derive(Default)]
pub(crate) struct EventQueue<M>(BTreeMap<EventKey<M>, M::Event>)
where
    M: Model;

impl<M> EventQueue<M>
where
    M: Model,
{
    pub(crate) fn insert(&mut self, event: Event<M>)
    where
        M::LogicalProcessId: Ord,
        M::VirtualTime: Ord,
    {
        self.0.insert(event.key, event.data);
    }

    pub(crate) fn pop_next(&mut self) -> Option<Event<M>>
    where
        M::VirtualTime: Ord,
        M::LogicalProcessId: Ord,
    {
        self.0.pop_first().map(|(key, data)| Event { key, data })
    }

    pub(crate) fn annihilate(&mut self, anti_event: AntiEvent<M>) -> bool
    where
        M::VirtualTime: Ord,
        M::LogicalProcessId: Ord,
    {
        self.0.remove(&anti_event.into()).is_some()
    }

    pub(crate) fn time_of_next_event(&self) -> Option<&M::VirtualTime>
    where
        M::VirtualTime: Ord,
        M::LogicalProcessId: Ord,
    {
        self.0.first_key_value().map(|(key, _)| key.time())
    }
}
