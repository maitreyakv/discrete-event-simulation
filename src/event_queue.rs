use std::collections::{BTreeMap, btree_map::Entry};

use crate::{Model, event::SequenceStamp};

pub(crate) struct EventQueue<M: Model>(BTreeMap<SequenceStamp<M>, (M::Event, M::LogicalProcessId)>);

impl<M: Model> EventQueue<M> {
    pub(crate) fn try_insert(
        &mut self,
        event: M::Event,
        sequence_stamp: SequenceStamp<M>,
        destination: M::LogicalProcessId,
    ) -> Result<(), DuplicateEventError> {
        match self.0.entry(sequence_stamp) {
            Entry::Vacant(v) => {
                v.insert((event, destination));
                Ok(())
            }
            Entry::Occupied(_) => Err(DuplicateEventError),
        }
    }

    pub(crate) fn pop_next(&mut self) -> Option<(SequenceStamp<M>, M::Event, M::LogicalProcessId)> {
        self.0
            .pop_first()
            .map(|(sequence_stamp, (event, destination))| (sequence_stamp, event, destination))
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
