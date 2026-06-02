use std::hash::Hash;

use crate::{Committable, scheduler::Scheduler};

pub trait Model: Sized {
    type LogicalProcessId: Ord + Hash + Clone;
    type VirtualTime: Ord + Clone;
    type State;
    type Event;
    type Output: Committable;
    type Error;

    fn initialize(_id: &Self::LogicalProcessId) -> (Self::State, Self::Event);

    fn start_time() -> Self::VirtualTime;

    fn process_event(
        scheduler: &mut Scheduler<Self>,
    ) -> Result<(Self::State, Self::Output), Self::Error>;
}
