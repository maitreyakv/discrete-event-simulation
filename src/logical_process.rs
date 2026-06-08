mod history;
mod scheduler;
mod set;

use history::History;
use history::Rollback;
pub use scheduler::Scheduler;

use crate::DesError;
use crate::event::Event;
use crate::{
    Committable, Model,
    event::{EventKey, EventQueue},
};

struct LogicalProcess<M>
where
    M: Model,
{
    state: M::State,
    history: History<M>,
    sequence_number: usize,
}

impl<M> LogicalProcess<M>
where
    M: Model,
{
    //     pub(crate) fn create(id: M::LogicalProcessId, state: M::State) -> Self {
    //         Self {
    //             id,
    //             state,
    //             history: Default::default(),
    //             sequence_number: 0,
    //         }
    //     }

    pub(crate) fn process_event(
        &mut self,
        event: Event<M>,
        scheduler: &mut Scheduler<M>,
    ) -> Result<(), DesError<M>>
    where
        M::VirtualTime: Ord + Clone,
        M::LogicalProcessId: Ord + Clone,
    {
        M::process_event(scheduler)
            .map_err(DesError::EventProcessFailure)
            .map(|(next_state, output)| {
                let prior_state = std::mem::replace(&mut self.state, next_state);
                let anti_events = scheduler.dispatch();
                self.history
                    .save_event(event, prior_state, output, anti_events);
            })
    }

    pub(crate) fn collect_fossils(&mut self, global_virtual_time: &M::VirtualTime)
    where
        M::VirtualTime: Ord,
        M::LogicalProcessId: Ord,
        M::Output: Committable,
    {
        self.history.collect_fossils(global_virtual_time);
    }

    pub(crate) fn rollback(&mut self, before: &EventKey<M>)
    where
        M::VirtualTime: Ord,
        M::LogicalProcessId: Ord,
    {
        self.history.rollback(before).for_each(
            |Rollback {
                 event,
                 prior_state,
                 anti_events,
             }| {
                self.state = prior_state;
                unimplemented!(/* re-queue event and send anti-events */)
            },
        );
    }
}
