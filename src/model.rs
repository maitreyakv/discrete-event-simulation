use crate::scheduler::Scheduler;

pub trait Model: Sized {
    type Timestamp: Ord + Copy;
    type State;
    type Event;

    fn init(logical_process_id: usize) -> Self;

    fn initial_state(&self) -> Self::State;

    fn initial_event(&self) -> Self::Event;

    fn initial_timestamp(&self) -> Self::Timestamp;

    fn process_event(&self, state: &Self::State, scheduler: &mut Scheduler<Self>) -> Self::State;
}
