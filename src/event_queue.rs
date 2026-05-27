use std::collections::{BTreeMap, btree_map::Entry};

use crate::{Model, event::Event};

pub(crate) struct EventQueue<M: Model>(BTreeMap<Event<M>, usize>);

impl<M: Model> EventQueue<M> {
    pub(crate) fn try_insert(
        &mut self,
        event: Event<M>,
        destination: usize,
    ) -> Result<(), DuplicateEventError> {
        match self.0.entry(event) {
            Entry::Vacant(v) => {
                v.insert(destination);
                Ok(())
            }
            Entry::Occupied(_) => Err(DuplicateEventError),
        }
    }

    pub(crate) fn pop_next(&mut self) -> Option<(Event<M>, usize)> {
        self.0.pop_first()
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
