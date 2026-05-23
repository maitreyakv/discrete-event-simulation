
// NOTE: We can potentially upgrade the schedule_event callback to an object that exposes a
// richer scheduling interface

pub trait Model {
    type Timestamp: Ord + Copy;
    type State;
    type Event;

    fn process_event<F>(
        logical_process_id: usize,
        state: &Self::State,
        event: &Self::Event,
        schedule_event: F,
    ) -> Self::State
    where 
        F: FnMut(Self::Event, Self::Timestamp, usize);
}
