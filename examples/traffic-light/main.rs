fn main() {
    discrete_event_simulation::run_single_thread::<TrafficLight>(vec![0].into_iter().collect(), 10)
        .unwrap();
}

struct TrafficLight;

impl discrete_event_simulation::Model for TrafficLight {
    type LogicalProcessId = usize;
    type VirtualTime = usize;
    type State = Color;
    type Event = ();
    type Error = std::convert::Infallible;

    fn initialize(_id: &Self::LogicalProcessId) -> (Self::State, Self::Event) {
        (Color::Red, ())
    }

    fn start_time() -> Self::VirtualTime {
        0
    }

    fn process_event(
        scheduler: &mut discrete_event_simulation::Scheduler<Self>,
    ) -> Result<Self::State, Self::Error> {
        let state = scheduler.state().to_owned();
        let (next, time) = match state {
            Color::Green => (Color::Yellow, 3),
            Color::Yellow => (Color::Red, 20),
            Color::Red => (Color::Green, 60),
        };
        let _ = scheduler.schedule_internal_event((), scheduler.time() + time);
        println!("{:?}: {state:?} -> {next:?}", scheduler.time());
        Ok(next)
    }
}

#[derive(Debug, Clone)]
enum Color {
    Green,
    Yellow,
    Red,
}
