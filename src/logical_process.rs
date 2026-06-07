mod history;

use history::History;
use history::Rollback;

use crate::{
    Committable, Model,
    event::{EventKey, EventQueue},
};

pub(crate) struct LogicalProcess<M>
where
    M: Model,
{
    pub(crate) id: M::LogicalProcessId,
    pub(crate) state: M::State,
    pub(crate) queue: EventQueue<M>,
    pub(crate) history: History<M>,
    pub(crate) sequence_number: usize,
}

impl<M> LogicalProcess<M>
where
    M: Model,
{
    fn create(id: M::LogicalProcessId, state: M::State) -> Self {
        Self {
            id,
            state,
            queue: Default::default(),
            history: Default::default(),
            sequence_number: 0,
        }
    }

    fn collect_fossils(&mut self, global_virtual_time: &M::VirtualTime)
    where
        M::VirtualTime: Ord,
        M::LogicalProcessId: Ord,
        M::Output: Committable,
    {
        self.history.collect_fossils(global_virtual_time);
    }

    fn rollback(&mut self, until: &EventKey<M>)
    where
        M::VirtualTime: Ord,
        M::LogicalProcessId: Ord,
    {
        self.history.rollback(until).for_each(
            |Rollback {
                 event,
                 prior_state,
                 anti_events,
             }| {
                self.state = prior_state;
                self.queue.insert(event);
                unimplemented!(/* send anti events */)
            },
        );
    }
}
