use std::hash::Hash;

use crate::scheduler::Scheduler;

pub trait Model: Sized {
    type LogicalProcessId: Ord + Hash + Clone;
    type VirtualTime: Ord;
    type State;
    type Event;
    type Error;

    fn initialize(_id: &Self::LogicalProcessId) -> (Self::State, Self::Event);

    fn start_time() -> Self::VirtualTime;

    fn process_event(scheduler: &mut Scheduler<Self>) -> Result<Self::State, Self::Error>;
}
