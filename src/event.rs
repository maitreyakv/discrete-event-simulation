use crate::{Model, logical_process::LogicalProcess};

// NOTE: We can maybe extract the age and sequence number out into a modular component that
// performs deterministic total ordering of events

pub(crate) struct EventKey<M: Model> {
    pub(crate) time: M::VirtualTime,
    pub(crate) age: u8,
    pub(crate) sender: M::LogicalProcessId,
    pub(crate) sequence_number: usize,
}

impl<M: Model> EventKey<M> {
    pub(crate) fn create_first(sender: M::LogicalProcessId) -> Self {
        Self {
            time: M::start_time(),
            age: 0,
            sender,
            sequence_number: 0,
        }
    }

    pub(crate) fn create_another(&self, time: M::VirtualTime, sender: &LogicalProcess<M>) -> Self {
        let age = if self.time == time { self.age + 1 } else { 0 };
        Self {
            time,
            age,
            sender: sender.id.clone(),
            sequence_number: sender.sequence_number,
        }
    }
}

// NOTE: We have to manually implement these traits because the derive macro would require
// `M: Ord` or `M: Clone`

impl<M: Model> PartialEq for EventKey<M> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
            && self.age == other.age
            && self.sender == other.sender
            && self.sequence_number == other.sequence_number
    }
}

impl<M: Model> Eq for EventKey<M> {}

impl<M: Model> PartialOrd for EventKey<M> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: Model> Ord for EventKey<M> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time
            .cmp(&other.time)
            .then(self.age.cmp(&other.age))
            .then(self.sender.cmp(&other.sender))
            .then(self.sequence_number.cmp(&other.sequence_number))
    }
}

impl<M: Model> Clone for EventKey<M> {
    fn clone(&self) -> Self {
        Self {
            time: self.time.clone(),
            age: self.age,
            sender: self.sender.clone(),
            sequence_number: self.sequence_number,
        }
    }
}
