use crate::{
    DesError, Model,
    event::{AntiEvent, Event, EventQueue},
};

pub struct Scheduler<'a, M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    // pub(crate) time: M::VirtualTime,
    pub(crate) local_event_queue: &'a EventQueue<M>,
    // pub(crate) scheduled_events: Vec<Event<M>>,
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

    // pub fn schedule_event(
    //     &mut self,
    //     event: M::Event,
    //     time: M::VirtualTime,
    //     destination: M::LogicalProcessId,
    // ) -> Result<(), DesError<M>> {
    //     if time < *self.time {
    //         return Err(DesError::CausalityViolation);
    //     }
    //     self.scheduled_events.push((event, time, destination));
    //     Ok(())
    // }

    // pub fn schedule_internal_event(
    //     &mut self,
    //     event: M::Event,
    //     time: M::VirtualTime,
    // ) -> Result<(), CausalityViolation> {
    //     self.schedule_event(event, time, self.logical_process_id().to_owned())
    // }
    //
    // pub(crate) fn new(
    //     logical_process: &'a mut LogicalProcess<M>,
    //     current_event: M::Event,
    //     current_event_key: EventKey<M>,
    //     event_queue: &'a mut EventQueue<M>,
    //     these_logical_processes: HashSet<M::LogicalProcessId>,
    // ) -> Self {
    //     Scheduler {
    //         logical_process,
    //         current_event,
    //         current_event_key,
    //         event_queue,
    //         these_logical_processes,
    //         scheduled_events: Default::default(),
    //     }
    // }
    //
    // pub(crate) fn process_event(mut self) -> Result<(), M::Error> {
    //     let (next_state, output) = M::process_event(&mut self)?;
    //     let Self {
    //         scheduled_events,
    //         current_event,
    //         current_event_key,
    //         logical_process,
    //         ..
    //     } = self;
    //     let scheduled_event_keys = scheduled_events
    //         .into_iter()
    //         .map(|(event, time, destination)| {
    //             let event_key = current_event_key.create_another(time, logical_process);
    //             if self.these_logical_processes.contains(&destination) {
    //                 self.event_queue
    //                     .insert(event, event_key.clone(), destination);
    //             } else {
    //                 unimplemented!(/* impl sending events to other workers via a postbox */)
    //             }
    //             logical_process.sequence_number += 1;
    //             event_key
    //         })
    //         .collect();
    //     let prior_state = std::mem::replace(&mut logical_process.state, next_state);
    //     logical_process.history.save_event(
    //         current_event,
    //         current_event_key,
    //         prior_state,
    //         output,
    //         scheduled_event_keys,
    //     );
    //     Ok(())
    // }
}
