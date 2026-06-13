use crate::{
    DesError, Model,
    event::{AntiEvent, Event},
    logical_process::set::LogicalProcessSet,
};
use std::hash::Hash;

pub struct Scheduler<'a, M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    pub(crate) current_event: &'a Event<M>,
    pub(crate) set: &'a mut LogicalProcessSet<M>,
    pub(crate) anti_events: Vec<AntiEvent<M>>,
}

impl<M> Scheduler<'_, M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    pub(crate) fn dispatch(&mut self) -> Vec<AntiEvent<M>> {
        todo!()
    }

    pub fn schedule_event(
        &mut self,
        event: M::Event,
        time: M::VirtualTime,
        destination: M::LogicalProcessId,
    ) -> Result<(), DesError<M>>
    where
        M::VirtualTime: Clone,
        M::LogicalProcessId: Clone + Hash,
    {
        if time < self.current_event.key.time {
            return Err(DesError::CausalityViolation);
        }
        let key = self.current_event.key.create_another(
            time,
            destination,
            self.current_event.key.location.clone(),
            self.set
                .id_to_logical_process
                .get(self.current_event.location())
                .ok_or_else(|| {
                    DesError::MissingLogicalProcess(self.current_event.location().clone())
                })?
                .sequence_number,
        );
        let event = Event { data: event, key };
        let anti_event = event.anti();
        if self
            .set
            .id_to_logical_process
            .contains_key(event.location())
        {
            self.set.event_queue.insert(event);
        } else {
            unimplemented!(/* sending external events not implemented */)
        }
        self.anti_events.push(anti_event);
        Ok(())
    }

    pub fn schedule_internal_event(
        &mut self,
        event: M::Event,
        time: M::VirtualTime,
    ) -> Result<(), DesError<M>>
    where
        M::VirtualTime: Clone,
        M::LogicalProcessId: Clone + Hash,
    {
        self.schedule_event(event, time, self.current_event.location().clone())
    }
}
