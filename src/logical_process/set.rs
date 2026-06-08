use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
};

use super::LogicalProcess;
use crate::{
    Committable, DesError, Model, Scheduler,
    event::{AntiEvent, Event, EventKey, EventQueue},
};

pub(crate) struct LogicalProcessSet<M>
where
    M: Model,
{
    id_to_logical_process: HashMap<M::LogicalProcessId, LogicalProcess<M>>,
    event_queue: EventQueue<M>,
    guard: BTreeSet<EventKey<M>>,
}

impl<M> LogicalProcessSet<M>
where
    M: Model,
{
    fn next_event(mut self) -> Option<NextEvent<M>>
    where
        M::VirtualTime: Ord,
        M::LogicalProcessId: Ord + Hash + Clone,
    {
        self.event_queue
            .pop_next()
            .map(move |next| NextEvent { next, set: self })
    }

    fn recv_event(&mut self, event: Event<M>) -> Result<(), DesError<M>>
    where
        M::LogicalProcessId: Ord + Hash + Clone,
    {
        if !self.guard.remove(&event.key) {
            self.id_to_logical_process
                .get_mut(event.location())
                .ok_or_else(|| DesError::MissingLogicalProcess(event.location().clone()))?
                .rollback(&event.key);
            self.event_queue.insert(event);
        }
        Ok(())
    }

    // pub(crate) fn receive_anti_event(
    //     &mut self,
    //     key: EventKey<M>,
    //     destination: M::LogicalProcessId,
    // ) -> Result<(), DesError<M>> {
    //     if self.event_queue.remove(&key) {
    //         return Ok(());
    //     }
    //     let logical_process = self
    //         .logical_processes
    //         .get_mut(&destination)
    //         .ok_or_else(|| DesError::MissingLogicalProcess(destination.clone()))?;
    //     if logical_process.history.contains_event(&key) {
    //         logical_process.rollback(&key, &mut self.event_queue);
    //         self.event_queue.remove(&key);
    //     } else {
    //         self.guard.insert(key);
    //     }
    //     Ok(())
    // }

    fn collect_fossils(&mut self, global_virtual_time: &M::VirtualTime)
    where
        M::LogicalProcessId: Ord,
        M::Output: Committable,
    {
        self.id_to_logical_process
            .values_mut()
            .for_each(|lp| lp.collect_fossils(global_virtual_time));
    }
}

struct NextEvent<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    next: Event<M>,
    set: LogicalProcessSet<M>,
}

impl<M> NextEvent<M>
where
    M: Model,
    M::VirtualTime: Ord + Clone,
    M::LogicalProcessId: Ord + Hash + Clone,
{
    fn process(mut self) -> Result<LogicalProcessSet<M>, DesError<M>> {
        let mut scheduler = Scheduler::new(self.set.event_queue);
        self.set
            .id_to_logical_process
            .get_mut(self.next.location())
            .ok_or_else(|| DesError::MissingLogicalProcess(self.next.location().clone()))?
            .process_event(self.next, &mut scheduler)
            .map(|_| LogicalProcessSet {
                event_queue: scheduler.local_event_queue,
                ..self.set
            })
    }
}
