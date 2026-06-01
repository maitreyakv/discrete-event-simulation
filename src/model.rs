use std::hash::Hash;

use crate::scheduler::Scheduler;

pub trait Model: Sized {
    type LogicalProcessId: Ord + Hash + Clone;
    type VirtualTime: Ord;
    type State;
    type Event;

    fn initial_state(id: &Self::LogicalProcessId) -> Self::State;

    fn initial_event(id: &Self::LogicalProcessId) -> Self::Event;

    fn initial_timestamp(id: &Self::LogicalProcessId) -> Self::VirtualTime;

    fn process_event(scheduler: &mut Scheduler<Self>) -> Self::State;
}
