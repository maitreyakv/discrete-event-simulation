use crate::{Committable, DesError, event_queue::EventQueue};

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
                Scheduler::new(
                    self.logical_processes
                        .get_mut(&destination)
                        .expect("event destination not in this set"),
                    &current_event,
                    &current_event_key,
                    &mut self.event_queue,
                    these_logical_processes,
                )
            };
            let (next_state, output) = M::process_event(&mut scheduler)?;
            let scheduled_event_keys = scheduler.send_scheduled_events();

            let this_logical_process = self
                .logical_processes
                .get_mut(&destination)
                .expect("event destination not in this set");
            let prior_state = std::mem::replace(&mut this_logical_process.state, next_state);
            this_logical_process.history.save_event(
                current_event,
                current_event_key,
                prior_state,
                output,
                scheduled_event_keys,
            );
        };

        Ok(())
    }

    pub(crate) fn time_of_next_event(&self) -> Option<&M::VirtualTime> {
        self.event_queue.time_of_next_event()
    }

    pub(crate) fn collect_fossils(&mut self, global_virtual_time: &M::VirtualTime) {
        self.logical_processes
            .values_mut()
            .for_each(|logical_process| {
                logical_process.history.collect_fossils(global_virtual_time)
            });
    }
}

pub(crate) struct LogicalProcess<M: Model> {
    pub(crate) id: M::LogicalProcessId,
    pub(crate) state: M::State,
    pub(crate) sequence_number: usize,
    history: History<M>,
}

pub(crate) struct History<M: Model> {
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
    fn save_event(
        &mut self,
        event: M::Event,
        event_key: EventKey<M>,
        prior_state: M::State,
        output: M::Output,
        scheduled_event_keys: Vec<EventKey<M>>,
    ) {
        self.records.push_back(Record {
            event,
            event_key,
            prior_state,
            output,
            scheduled_event_keys,
        });
    }

    fn collect_fossils(&mut self, global_virtual_time: &M::VirtualTime) {
        if let Some(Record { output, .. }) = self
            .records
            .pop_front_if(|r| r.event_key.time < *global_virtual_time)
        {
            output.commit();
        }
    }
}

#[allow(dead_code)]
struct Record<M: Model> {
    event: M::Event,
    event_key: EventKey<M>,
    prior_state: M::State,
    output: M::Output,
    scheduled_event_keys: Vec<EventKey<M>>,
}
