use std::cmp::Ordering;

use crate::Model;

// NOTE: We can maybe extract the age and sequence number out into a modular component that
// performs deterministic total ordering of events

#[derive(Debug)]
pub(crate) struct Event<M: Model> {
    pub(crate) data: M::Event,
    pub(crate) timestamp: M::Timestamp,
    pub(crate) sequence_stamp: SequenceStamp,
}

impl<M: Model> Ord for Event<M> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then(self.sequence_stamp.cmp(&other.sequence_stamp))
    }
}

impl<M: Model> PartialOrd for Event<M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.timestamp
                .cmp(&other.timestamp)
                .then(self.sequence_stamp.cmp(&other.sequence_stamp)),
        )
    }
}

impl<M: Model> PartialEq for Event<M> {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp && self.sequence_stamp == other.sequence_stamp
    }
}

impl<M: Model> Eq for Event<M> {}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub(crate) struct SequenceStamp {
    pub(crate) age: u8,
    pub(crate) sender: usize,
    pub(crate) sequence_number: usize,
}
