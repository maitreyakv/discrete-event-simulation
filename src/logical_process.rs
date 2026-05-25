use std::collections::VecDeque;

use crate::{Model, event::Event, scheduler::Scheduler};

pub(crate) struct LogicalProcess<M: Model> {
    model: M,
    state: M::State,
    history: History<M>,
    scheduler: Scheduler<M>,
}

impl<M: Model> LogicalProcess<M> {
    pub(crate) fn new(id: usize) -> Self {
        let model = M::init(id);
        let initial_state = model.initial_state();
        let initial_event = model.initial_event();
        let initial_timestamp = model.initial_timestamp();
        Self {
            model,
            state: initial_state,
            history: History::new(),
            scheduler: Scheduler::new(id, initial_event, initial_timestamp),
        }
    }

    pub(crate) fn next_event(&self) -> Option<&Event<M>> {
        self.scheduler.next_event()
    }

    pub(crate) fn process_next_event(&mut self) {
        if self.next_event().is_some() {
            let next_state = self.model.process_event(&self.state, &mut self.scheduler);
            let processed_event = self.scheduler.pop_next_event().unwrap();
            let prior_state = std::mem::replace(&mut self.state, next_state);
            self.history.save_event(processed_event, prior_state);
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
