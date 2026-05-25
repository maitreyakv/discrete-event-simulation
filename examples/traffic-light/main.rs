fn main() {
    discrete_event_simulation::SingleThreadWorker::<TrafficLight>::new()
        .with_logical_process(0)
        .run(10);
}

struct TrafficLight;

impl discrete_event_simulation::Model for TrafficLight {
    type Timestamp = usize;
    type State = Color;
    type Event = ();

    fn init(_: usize) -> Self {
        Self
    }

    fn initial_state(&self) -> Self::State {
        Color::Red
    }

    fn initial_event(&self) -> Self::Event {}

    fn initial_timestamp(&self) -> Self::Timestamp {
        0
    }

    fn process_event(
        &self,
        state: &Self::State,
        scheduler: &mut discrete_event_simulation::Scheduler<Self>,
    ) -> Self::State {
        let (next, time) = match state {
            Color::Green => (Color::Yellow, 3),
            Color::Yellow => (Color::Red, 20),
            Color::Red => (Color::Green, 60),
        };
        scheduler
            .schedule_internal_event((), scheduler.timestamp().unwrap() + time)
            .unwrap();
        println!(
            "{:?}: {state:?} -> {next:?}",
            scheduler.timestamp().unwrap()
        );
        next
    }
}

#[derive(Debug)]
enum Color {
    Green,
    Yellow,
    Red,
}
