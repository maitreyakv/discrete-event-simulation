use std::hash::Hash;

use crate::scheduler::Scheduler;

pub trait Model: Sized {
    type LogicalProcessId: Ord + Hash + Clone;
    type VirtualTime: Ord;
    type State;
    type Event;

    fn init_state(id: &Self::LogicalProcessId) -> Self::State;

    fn init_event(id: &Self::LogicalProcessId) -> Self::Event;

    fn init_time(id: &Self::LogicalProcessId) -> Self::VirtualTime;

    fn process_event(scheduler: &mut Scheduler<Self>) -> Self::State;
}
