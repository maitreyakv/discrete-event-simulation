
use discrete_event_simulation::{Model};

fn main() {
}


struct TrafficLight;

impl Model for TrafficLight {
    type Timestamp = usize;
    type State = Color;
    type Event = ();

    fn process_event<F>(
        logical_process_id: usize,
        state: &Self::State,
        _event: &Self::Event,
        mut schedule_event: F
    ) -> Self::State 
    where
        F: FnMut(Self::Event, Self::Timestamp, usize),
    {
        match state {
            Color::Green => {
                schedule_event((), 3, logical_process_id);
                Color::Yellow
            },
            Color::Yellow => {
                schedule_event((), 20, logical_process_id);
                Color::Red
            },
            Color::Red => {
                schedule_event((), 60, logical_process_id);
                Color::Green
            },
        }
    }
}

enum Color {
    Green,
    Yellow,
    Red,
}
