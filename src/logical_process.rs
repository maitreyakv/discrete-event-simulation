use crate::{Committable, DesError, event_queue::EventQueue};

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

use crate::{Model, event::EventKey, scheduler::Scheduler};

pub(crate) struct LogicalProcessSet<M: Model> {
    logical_processes: BTreeMap<M::LogicalProcessId, LogicalProcess<M>>,
    event_queue: EventQueue<M>,
    guard: BTreeSet<EventKey<M>>,
}

impl<M: Model> LogicalProcessSet<M> {
    pub(crate) fn from_ids(ids: HashSet<M::LogicalProcessId>) -> Self {
        let mut event_queue = EventQueue::default();
        let logical_processes = ids
            .into_iter()
            .map(|id| {
                let (state, event) = M::initialize(&id);
                event_queue.insert(event, EventKey::create_first(id.clone()), id.clone());
                let logical_process = LogicalProcess::create(id.clone(), state);
                (id, logical_process)
            })
            .collect();
        Self {
            logical_processes,
            event_queue,
            guard: Default::default(),
        }
    }

    pub(crate) fn process_next_event(&mut self) -> Result<(), DesError<M>> {
        if let Some((current_event_key, current_event, destination)) = self.event_queue.pop_next() {
            let these_logical_processes = self.logical_processes.keys().cloned().collect();
            Scheduler::new(
                self.logical_processes
                    .get_mut(&destination)
                    .expect("event destination not in this set"),
                current_event,
                current_event_key,
                &mut self.event_queue,
                these_logical_processes,
            )
            .process_event()
            .map_err(DesError::EventProcessFailure)?
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

    pub(crate) fn receive_event(
        &mut self,
        event: M::Event,
        key: EventKey<M>,
        destination: M::LogicalProcessId,
    ) -> Result<(), DesError<M>> {
        if !self.guard.remove(&key) {
            self.logical_processes
                .get_mut(&destination)
                .ok_or_else(|| DesError::MissingLogicalProcess(destination.clone()))?
                .rollback(&key, &mut self.event_queue);
            self.event_queue.insert(event, key, destination);
        }
        Ok(())
    }

    pub(crate) fn receive_anti_event(
        &mut self,
        key: EventKey<M>,
        destination: M::LogicalProcessId,
    ) -> Result<(), DesError<M>> {
        if self.event_queue.remove(&key) {
            return Ok(());
        }
        let logical_process = self
            .logical_processes
            .get_mut(&destination)
            .ok_or_else(|| DesError::MissingLogicalProcess(destination.clone()))?;
        if logical_process.history.contains_event(&key) {
            logical_process.rollback(&key, &mut self.event_queue);
            self.event_queue.remove(&key);
        } else {
            self.guard.insert(key);
        }
        Ok(())
    }
}

pub(crate) struct LogicalProcess<M: Model> {
    pub(crate) id: M::LogicalProcessId,
    pub(crate) state: M::State,
    pub(crate) sequence_number: usize,
    pub(crate) history: History<M>,
}

impl<M: Model> LogicalProcess<M> {
    fn create(id: M::LogicalProcessId, state: M::State) -> Self {
        Self {
            id,
            state,
            sequence_number: 0,
            history: Default::default(),
        }
    }

    fn rollback(&mut self, until: &EventKey<M>, event_queue: &mut EventQueue<M>) {
        self.history.rollback(until).for_each(
            |(
                event_key,
                Record {
                    prior_state, event, ..
                },
            )| {
                self.state = prior_state;
                event_queue.insert(event, event_key, self.id.clone());
                unimplemented!(/* send anti events */)
            },
        );
    }
}

pub(crate) struct History<M: Model> {
    records: BTreeMap<EventKey<M>, Record<M>>,
}

impl<M: Model> Default for History<M> {
    fn default() -> Self {
        Self {
            records: Default::default(),
        }
    }
}

impl<M: Model> History<M> {
    pub(crate) fn save_event(
        &mut self,
        event: M::Event,
        event_key: EventKey<M>,
        prior_state: M::State,
        output: M::Output,
        scheduled_event_keys: Vec<EventKey<M>>,
    ) {
        self.records.insert(
            event_key,
            Record {
                event,
                prior_state,
                output,
                scheduled_event_keys,
            },
        );
    }

    fn contains_event(&self, key: &EventKey<M>) -> bool {
        self.records.contains_key(key)
    }

    fn rollback(&mut self, until: &EventKey<M>) -> impl Iterator<Item = (EventKey<M>, Record<M>)> {
        std::iter::from_fn(move || {
            if let Some((key, _)) = self.records.last_key_value()
                && key >= until
            {
                self.records.pop_last()
            } else {
                None
            }
        })
    }

    fn collect_fossils(&mut self, global_virtual_time: &M::VirtualTime) {
        while let Some((key, _)) = self.records.first_key_value()
            && key.time < *global_virtual_time
        {
            self.records.pop_first().unwrap().1.output.commit();
        }
    }
}

#[allow(dead_code)]
struct Record<M: Model> {
    event: M::Event,
    prior_state: M::State,
    output: M::Output,
    scheduled_event_keys: Vec<EventKey<M>>,
}
