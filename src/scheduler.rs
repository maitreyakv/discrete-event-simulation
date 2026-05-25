use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::{
    Model,
    event::{Event, SequenceStamp},
};

pub struct Scheduler<M: Model> {
    event_queue: BinaryHeap<Reverse<Event<M>>>,
    sender: usize,
    sequence_number: usize,
}

impl<M: Model> Scheduler<M> {
    pub fn logical_process_id(&self) -> usize {
        self.sender
    }

    pub fn timestamp(&self) -> Option<M::Timestamp> {
        self.event_queue.peek().map(|Reverse(e)| e.timestamp)
    }

    pub fn event(&self) -> Option<&M::Event> {
        self.event_queue.peek().map(|Reverse(e)| &e.data)
    }

    pub fn schedule_event(
        &mut self,
        data: M::Event,
        timestamp: M::Timestamp,
        destination: usize,
    ) -> Result<(), ()> {
        let source = self.next_event().unwrap();
        let sequence_stamp = SequenceStamp {
            age: if source.timestamp == timestamp {
                source.sequence_stamp.age + 1
            } else {
                0
            },
            sender: self.sender,
            sequence_number: self.sequence_number,
        };
        let event = Event::<M> {
            data,
            timestamp,
            sequence_stamp,
        };

        if &event < source {
            return Err(());
        }

        if destination == self.sender {
            self.event_queue.push(Reverse(event));
        } else {
            unimplemented!()
        }

        self.sequence_number += 1;
        Ok(())
    }

    pub fn schedule_internal_event(
        &mut self,
        event: M::Event,
        timestamp: M::Timestamp,
    ) -> Result<(), ()> {
        self.schedule_event(event, timestamp, self.sender)
    }

    pub(crate) fn new(
        sender: usize,
        initial_event: M::Event,
        initial_timestamp: M::Timestamp,
    ) -> Self {
        let mut event_queue = BinaryHeap::default();
        event_queue.push(Reverse(Event {
            data: initial_event,
            timestamp: initial_timestamp,
            sequence_stamp: SequenceStamp {
                age: 0,
                sender,
                sequence_number: 0,
            },
        }));
        Self {
            event_queue,
            sender,
            sequence_number: 1,
        }
    }

    pub(crate) fn next_event(&self) -> Option<&Event<M>> {
        self.event_queue.peek().map(|r| &r.0)
    }

    pub(crate) fn pop_next_event(&mut self) -> Option<Event<M>> {
        self.event_queue.pop().map(|r| r.0)
    }
}
