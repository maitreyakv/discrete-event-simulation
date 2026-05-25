use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::{
    Model,
    event::{Event, SequenceStamp},
};

pub struct Scheduler<'a, M: Model> {
    pub(crate) current_event: &'a Event<M>,
    pub(crate) event_queue: &'a mut BinaryHeap<Reverse<Event<M>>>,
    pub(crate) sender: &'a usize,
    pub(crate) sequence_number: &'a mut usize,
}

impl<M: Model> Scheduler<'_, M> {
    pub fn logical_process_id(&self) -> &usize {
        self.sender
    }

    pub fn timestamp(&self) -> &M::Timestamp {
        &self.current_event.timestamp
    }

    pub fn current_event(&self) -> &M::Event {
        &self.current_event.data
    }

    pub fn schedule_event(
        &mut self,
        data: M::Event,
        timestamp: M::Timestamp,
        destination: usize,
    ) -> Result<(), SchedulerError> {
        if timestamp < self.current_event.timestamp {
            return Err(SchedulerError::CausalityViolation);
        }

        let sequence_stamp = SequenceStamp {
            age: if self.current_event.timestamp == timestamp {
                self.current_event.sequence_stamp.age + 1
            } else {
                0
            },
            sender: *self.sender,
            sequence_number: *self.sequence_number,
        };
        let event = Event::<M> {
            data,
            timestamp,
            sequence_stamp,
        };

        if destination == *self.sender {
            self.event_queue.push(Reverse(event));
        } else {
            unimplemented!()
        }

        *self.sequence_number += 1;
        Ok(())
    }

    pub fn schedule_internal_event(
        &mut self,
        event: M::Event,
        timestamp: M::Timestamp,
    ) -> Result<(), SchedulerError> {
        self.schedule_event(event, timestamp, *self.sender)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SchedulerError {
    #[error("event is being scheduled in the past")]
    CausalityViolation,
}
