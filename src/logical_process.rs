use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use crate::{
    Model,
    event::{Event, SequenceStamp},
    scheduler::Scheduler,
};

type BinaryMinHeap<T> = BinaryHeap<Reverse<T>>;

pub(crate) struct LogicalProcess<M: Model> {
    id: usize,
    model: M,
    state: M::State,
    history: History<M>,
    event_queue: BinaryMinHeap<Event<M>>,
    sequence_number: usize,
}

impl<M: Model> LogicalProcess<M> {
    pub(crate) fn new(id: usize) -> Self {
        let model = M::init(id);
        let initial_state = model.initial_state();
        let initial_event = model.initial_event();
        let initial_timestamp = model.initial_timestamp();

        let mut event_queue = BinaryMinHeap::default();
        event_queue.push(Reverse(Event {
            data: initial_event,
            timestamp: initial_timestamp,
            sequence_stamp: SequenceStamp {
                age: 0,
                sender: id,
                sequence_number: 0,
            },
        }));

        Self {
            id,
            model,
            state: initial_state,
            history: History::new(),
            event_queue,
            sequence_number: 1,
        }
    }

    pub(crate) fn next_event(&self) -> Option<&Event<M>> {
        self.event_queue.peek().map(|r| &r.0)
    }

    pub(crate) fn process_next_event(&mut self) {
        if let Some(Reverse(current_event)) = self.event_queue.pop() {
            let mut scheduler = Scheduler {
                current_event: &current_event,
                event_queue: &mut self.event_queue,
                sender: &self.id,
                sequence_number: &mut self.sequence_number,
            };
            let next_state = self.model.process_event(&self.state, &mut scheduler);
            let prior_state = std::mem::replace(&mut self.state, next_state);
            self.history.save_event(current_event, prior_state);
        };
    }
}

struct History<M: Model> {
    records: VecDeque<Record<M>>,
}

impl<M: Model> History<M> {
    fn new() -> Self {
        Self {
            records: VecDeque::default(),
        }
    }

    fn save_event(&mut self, event: Event<M>, prior_state: M::State) {
        self.records.push_back(Record { event, prior_state });
    }
}

#[allow(dead_code)]
struct Record<M: Model> {
    event: Event<M>,
    prior_state: M::State,
}
