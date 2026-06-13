use crate::Scheduler;

pub trait Model: Sized {
    type LogicalProcessId;
    type VirtualTime: Ord;
    type State;
    type Event;
    type Output;
    type Error;

    fn initialize(id: &Self::LogicalProcessId) -> (Self::State, Self::Event);

    fn start_time() -> Self::VirtualTime;

    fn process_event(
        scheduler: &mut Scheduler<Self>,
    ) -> Result<(Self::State, Self::Output), Self::Error>
    where
        Self::VirtualTime: Ord,
        Self::LogicalProcessId: Ord;
}

pub trait Committable {
    fn commit(self);
}
