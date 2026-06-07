pub trait Model: Sized {
    type LogicalProcessId;
    type VirtualTime;
    type State;
    type Event;
    type Output;
    type Error;

    fn initialize(id: &Self::LogicalProcessId) -> (Self::State, Self::Event);

    fn start_time() -> Self::VirtualTime;

    fn process_event() -> Result<(Self::State, Self::Output), Self::Error>;
}

pub trait Committable {
    fn commit(self);
}
