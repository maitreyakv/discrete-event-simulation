use std::collections::HashSet;

use crate::{
    Model,
    event::EventKey,
    event_queue::{self, EventQueue},
    logical_process::LogicalProcess,
};

pub struct Scheduler<'a, M: Model> {
    pub(crate) logical_process: &'a mut LogicalProcess<M>,
    pub(crate) current_event: &'a M::Event,
    pub(crate) current_event_key: &'a EventKey<M>,
    pub(crate) event_queue: &'a mut EventQueue<M>,
    pub(crate) these_logical_processes: HashSet<M::LogicalProcessId>,
}

impl<M: Model> Scheduler<'_, M> {
    pub fn logical_process_id(&self) -> &M::LogicalProcessId {
        &self.logical_process.id
    }

    pub fn time(&self) -> &M::VirtualTime {
        &self.current_event_key.time
    }

    pub fn state(&self) -> &M::State {
        &self.logical_process.state
    }

    pub fn event(&self) -> &M::Event {
        self.current_event
    }

    pub fn schedule_event(
        &mut self,
        event: M::Event,
        time: M::VirtualTime,
        destination: M::LogicalProcessId,
    ) -> Result<(), SchedulerError> {
        if time < *self.time() {
            return Err(SchedulerError::CausalityViolation);
        }

        let event_key = {
            let age = if *self.time() == time {
                self.current_event_key.age + 1
            } else {
                0
            };
            EventKey {
                time,
                age,
                sender: self.logical_process_id().to_owned(),
                sequence_number: self.logical_process.sequence_number,
            }
        };

        if self.these_logical_processes.contains(&destination) {
            self.event_queue
                .try_insert(event, event_key, destination)
                .map_err(SchedulerError::from)?
        } else {
            unimplemented!()
        }

        self.logical_process.sequence_number += 1;
        Ok(())
    }

    pub fn schedule_internal_event(
        &mut self,
        event: M::Event,
        time: M::VirtualTime,
    ) -> Result<(), SchedulerError> {
        self.schedule_event(event, time, self.logical_process_id().to_owned())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SchedulerError {
    #[error("event is being scheduled in the past")]
    CausalityViolation,

    #[error("event has already been scheduled")]
    DuplicateEvent(#[from] event_queue::DuplicateEventError),
}
