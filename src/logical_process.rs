use crate::{DesError, event_queue::EventQueue};

use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::{Model, event::EventKey, scheduler::Scheduler};

pub(crate) struct LogicalProcessSet<M: Model> {
    logical_processes: BTreeMap<M::LogicalProcessId, LogicalProcess<M>>,
    event_queue: EventQueue<M>,
}

impl<M: Model> LogicalProcessSet<M> {
    pub(crate) fn from_ids(ids: HashSet<M::LogicalProcessId>) -> Self {
        let mut event_queue = EventQueue::default();

        let logical_processes = ids
            .into_iter()
            .map(|id| {
                let (state, event) = M::initialize(&id);
                event_queue.insert(
                    event,
                    EventKey {
                        time: M::start_time(),
                        age: 0,
                        sender: id.to_owned(),
                        sequence_number: 0,
                    },
                    id.to_owned(),
                );

                let logical_process = LogicalProcess {
                    id: id.to_owned(),
                    state,
                    sequence_number: 1,
                    history: Default::default(),
                };

                (id, logical_process)
            })
            .collect();

        Self {
            logical_processes,
            event_queue,
        }
    }

    pub(crate) fn process_next_event(&mut self) -> Result<(), DesError<M::Error>> {
        if let Some((current_event_key, current_event, destination)) = self.event_queue.pop_next() {
            let mut scheduler = {
                let these_logical_processes = self.logical_processes.keys().cloned().collect();
                Scheduler {
                    logical_process: self
                        .logical_processes
                        .get_mut(&destination)
                        .expect("event destination not in this set"),
                    current_event: &current_event,
                    current_event_key: &current_event_key,
                    event_queue: &mut self.event_queue,
                    these_logical_processes,
                }
            };
            let next_state =
                M::process_event(&mut scheduler).map_err(|e| DesError::EventProcessFailure(e))?;

            let this_logical_process = self
                .logical_processes
                .get_mut(&destination)
                .expect("event destination not in this set");
            let prior_state = std::mem::replace(&mut this_logical_process.state, next_state);
            this_logical_process.history.save_event(
                current_event,
                current_event_key.time,
                prior_state,
            );
        };

        Ok(())
    }
}

pub(crate) struct LogicalProcess<M: Model> {
    pub(crate) id: M::LogicalProcessId,
    pub(crate) state: M::State,
    pub(crate) sequence_number: usize,
    history: History<M>,
}

struct History<M: Model> {
    records: VecDeque<Record<M>>,
}

impl<M: Model> Default for History<M> {
    fn default() -> Self {
        Self {
            records: VecDeque::default(),
        }
    }
}

impl<M: Model> History<M> {
    fn save_event(&mut self, event: M::Event, time: M::VirtualTime, prior_state: M::State) {
        self.records.push_back(Record {
            event,
            time,
            prior_state,
        });
    }
}

#[allow(dead_code)]
struct Record<M: Model> {
    event: M::Event,
    time: M::VirtualTime,
    prior_state: M::State,
}
