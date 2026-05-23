use std::collections::{BinaryHeap, VecDeque};

use crate::{Model, event::{Event, SequenceStamp}};

struct LogicalProcess<M: Model> {
    id: usize,
    state: M::State,
    history: VecDeque<Record<M>>,
    scheduler: Scheduler<M>,
}

impl<M: Model> LogicalProcess<M> {
    pub fn process_all_queued_events(&mut self) {
        let Self { id, state, history, scheduler } = self;

        while let Some(event) = scheduler.next_event() {
            let mut schedule_event = |data, timestamp, destination| {
                scheduler.schedule_event(
                    data, timestamp, destination, &event
                );
            };

            let new_state = M::process_event(*id, &state, &event.data, &mut schedule_event);
            let prior_state = std::mem::replace(state, new_state);
            history.push_back(Record { event, prior_state });
        }
    }
}


struct Scheduler<M: Model> {
    sender: usize,
    sequence_number: usize,
    event_queue: BinaryHeap<Event<M>>,
}

impl<M: Model> Scheduler<M> {
    fn next_event(&mut self) -> Option<Event<M>> {
        self.event_queue.pop()
    }

    fn schedule_event(
        &mut self, 
        data: M::Event,
        timestamp: M::Timestamp,
        destination: usize,
        source: &Event<M>,
    ) {
        let Self { sender, sequence_number, event_queue } = self;

        let sequence_stamp = SequenceStamp {
            age: if source.timestamp == timestamp { source.sequence_stamp.age + 1 } else { 0 },
            sender: *sender,
            sequence_number: *sequence_number,
        };
        let event = Event { data, timestamp, sequence_stamp };

        if destination == *sender {
            event_queue.push(event);
        } else {
            todo!()
        }

        *sequence_number += 1;
    }

    fn send_anti_event() {
        todo!()

        // NOTE: Reminder to decrement n_sent_events
    }
}


struct Record<M: Model> {
    event: Event<M>,
    prior_state: M::State,
}
