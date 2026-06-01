use discrete_event_simulation::run_single_thread;

fn main() {
    run_single_thread::<TrafficLight>(vec![0].into_iter().collect(), 10);
}

struct TrafficLight;

impl discrete_event_simulation::Model for TrafficLight {
    type LogicalProcessId = usize;
    type VirtualTime = usize;
    type State = Color;
    type Event = ();

    fn init_state(_id: &Self::LogicalProcessId) -> Self::State {
        Color::Red
    }

    fn init_event(_id: &Self::LogicalProcessId) -> Self::Event {}

    fn init_time(_id: &Self::LogicalProcessId) -> Self::VirtualTime {
        0
    }

    fn process_event(scheduler: &mut discrete_event_simulation::Scheduler<Self>) -> Self::State {
        let state = scheduler.state().to_owned();
        let (next, time) = match state {
            Color::Green => (Color::Yellow, 3),
            Color::Yellow => (Color::Red, 20),
            Color::Red => (Color::Green, 60),
        };
        scheduler
            .schedule_internal_event((), scheduler.time() + time)
            .unwrap();
        println!("{:?}: {state:?} -> {next:?}", scheduler.time());
        next
    }
}

#[derive(Debug, Clone)]
enum Color {
    Green,
    Yellow,
    Red,
}
