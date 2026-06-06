use std::collections::HashSet;

use crate::{Model, event::EventKey, event_queue::EventQueue, logical_process::LogicalProcess};

pub struct Scheduler<'a, M: Model> {
    pub(crate) logical_process: &'a mut LogicalProcess<M>,
    pub(crate) current_event: M::Event,
    pub(crate) current_event_key: EventKey<M>,
    pub(crate) event_queue: &'a mut EventQueue<M>,
    pub(crate) these_logical_processes: HashSet<M::LogicalProcessId>,
    pub(crate) scheduled_events: Vec<(M::Event, M::VirtualTime, M::LogicalProcessId)>,
}

impl<'a, M: Model> Scheduler<'a, M> {
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
        &self.current_event
    }

    pub fn schedule_event(
        &mut self,
        event: M::Event,
        time: M::VirtualTime,
        destination: M::LogicalProcessId,
    ) -> Result<(), CausalityViolation> {
        if time < *self.time() {
            return Err(CausalityViolation);
        }
        self.scheduled_events.push((event, time, destination));
        Ok(())
    }

    pub fn schedule_internal_event(
        &mut self,
        event: M::Event,
        time: M::VirtualTime,
    ) -> Result<(), CausalityViolation> {
        self.schedule_event(event, time, self.logical_process_id().to_owned())
    }

    pub(crate) fn new(
        logical_process: &'a mut LogicalProcess<M>,
        current_event: M::Event,
        current_event_key: EventKey<M>,
        event_queue: &'a mut EventQueue<M>,
        these_logical_processes: HashSet<M::LogicalProcessId>,
    ) -> Self {
        Scheduler {
            logical_process,
            current_event,
            current_event_key,
            event_queue,
            these_logical_processes,
            scheduled_events: Default::default(),
        }
    }

    pub(crate) fn process_event(mut self) -> Result<(), M::Error> {
        let (next_state, output) = M::process_event(&mut self)?;
        let Self {
            scheduled_events,
            current_event,
            current_event_key,
            logical_process,
            ..
        } = self;
        let scheduled_event_keys = scheduled_events
            .into_iter()
            .map(|(event, time, destination)| {
                let event_key = current_event_key.create_another(time, logical_process);
                if self.these_logical_processes.contains(&destination) {
                    self.event_queue
                        .insert(event, event_key.clone(), destination);
                } else {
                    unimplemented!()
                }
                logical_process.sequence_number += 1;
                event_key
            })
            .collect();
        let prior_state = std::mem::replace(&mut logical_process.state, next_state);
        logical_process.history.save_event(
            current_event,
            current_event_key,
            prior_state,
            output,
            scheduled_event_keys,
        );
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("event was scheduled in the past")]
pub struct CausalityViolation;
