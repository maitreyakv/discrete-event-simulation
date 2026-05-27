use crate::event_queue::EventQueue;

use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::{
    Model,
    event::{Event, SequenceStamp},
    scheduler::Scheduler,
};

pub(crate) struct LogicalProcessSet<M: Model> {
    logical_processes: BTreeMap<usize, LogicalProcess<M>>,
    event_queue: EventQueue<M>,
}

impl<M: Model> LogicalProcessSet<M> {
    pub(crate) fn from_ids(ids: HashSet<usize>) -> Self {
        let mut event_queue = EventQueue::default();
        let logical_processes = ids
            .into_iter()
            .map(|id| {
                let model = M::init(id);
                let initial_state = model.initial_state();
                let initial_event = Event {
                    data: model.initial_event(),
                    timestamp: model.initial_timestamp(),
                    sequence_stamp: SequenceStamp {
                        age: 0,
                        sender: id,
                        sequence_number: 0,
                    },
                };
                event_queue
                    .try_insert(initial_event, id)
                    .expect("duplicate event was produced during initialization");
                let logical_process = LogicalProcess {
                    model,
                    state: initial_state,
                    sequence_number: 1,
                };
                (id, logical_process)
            })
            .collect();
        Self {
            logical_processes,
            event_queue,
        }
    }

    pub(crate) fn contains(&self, id: &usize) -> bool {
        self.logical_processes.contains_key(id)
    }

    // pub(crate) fn next_event(&self) -> Option<&Event<M>> {
    //     self.event_queue.peek().map(|r| &r.0)
    // }

    pub(crate) fn process_next_event(&mut self) {
        if let Some((current_event, destination)) = self.event_queue.pop_next() {
            // let mut scheduler = Scheduler {
            //     current_event: &current_event,
            //     event_queue: &mut self.event_queue,
            //     sender: &destination,
            //     sequence_number: &mut self.logical_processes.get_mut(destination).,
            // };
            //     let next_state = self.model.process_event(&self.state, &mut scheduler);
            //     let prior_state = std::mem::replace(&mut self.state, next_state);
            //     self.history.save_event(current_event, prior_state);
        };
    }
}

struct LogicalProcess<M: Model> {
    model: M,
    state: M::State,
    sequence_number: usize,
}

// struct History<M: Model> {
//     records: VecDeque<Record<M>>,
// }
//
// impl<M: Model> History<M> {
//     fn new() -> Self {
//         Self {
//             records: VecDeque::default(),
//         }
//     }
//
//     fn save_event(&mut self, event: Event<M>, prior_state: M::State) {
//         self.records.push_back(Record { event, prior_state });
//     }
// }
//
// #[allow(dead_code)]
// struct Record<M: Model> {
//     event: Event<M>,
//     prior_state: M::State,
// }
