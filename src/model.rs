
// NOTE: We can potentially upgrade the schedule_event callback to an object that exposes a
// richer scheduling interface

pub trait Model {
    type Timestamp: Ord;
    type State;
    type Event;

    fn process_event(
        logical_process_id: usize,
        state: &Self::State,
        event: &Self::Event,
        schedule_event: fn(Self::Event, Self::Timestamp, usize)
    ) -> Self::State;
}
