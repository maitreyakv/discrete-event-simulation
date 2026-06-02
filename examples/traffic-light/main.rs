fn main() {
    discrete_event_simulation::run_single_thread::<TrafficLight>(
        vec![0].into_iter().collect(),
        600,
    )
    .unwrap();
}

struct TrafficLight;

impl discrete_event_simulation::Model for TrafficLight {
    type LogicalProcessId = usize;
    type VirtualTime = usize;
    type State = Color;
    type Event = ();
    type Output = Change;
    type Error = std::convert::Infallible;

    fn initialize(_id: &Self::LogicalProcessId) -> (Self::State, Self::Event) {
        (Color::Red, ())
    }

    fn start_time() -> Self::VirtualTime {
        0
    }

    fn process_event(
        scheduler: &mut discrete_event_simulation::Scheduler<Self>,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        let old = scheduler.state().to_owned();
        let time = scheduler.time().to_owned();
        let (new, duration) = match old {
            Color::Green => (Color::Yellow, 3),
            Color::Yellow => (Color::Red, 20),
            Color::Red => (Color::Green, 60),
        };
        let _ = scheduler.schedule_internal_event((), time + duration);
        Ok((new, Change { old, new, time }))
    }
}

struct Change {
    old: Color,
    new: Color,
    time: usize,
}

impl discrete_event_simulation::Committable for Change {
    fn commit(self) {
        let Change { old, new, time } = self;
        println!("changed from {old:?} -> {new:?} at {time} seconds");
    }
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Green,
    Yellow,
    Red,
}
