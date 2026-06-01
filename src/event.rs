use crate::Model;

// NOTE: We can maybe extract the age and sequence number out into a modular component that
// performs deterministic total ordering of events

pub(crate) struct EventKey<M: Model> {
    pub(crate) timestamp: M::VirtualTime,
    pub(crate) age: u8,
    pub(crate) sender: M::LogicalProcessId,
    pub(crate) sequence_number: usize,
}

// NOTE: We have to manually implement these traits because the derive macro would require `M: Ord`

impl<M: Model> PartialEq for EventKey<M> {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
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
        self.timestamp
            .cmp(&other.timestamp)
            .then(self.age.cmp(&other.age))
            .then(self.sender.cmp(&other.sender))
            .then(self.sequence_number.cmp(&other.sequence_number))
    }
}
