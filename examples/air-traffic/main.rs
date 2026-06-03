use discrete_event_simulation::{self as des};

use std::{collections::VecDeque, str::FromStr};

fn main() {
    // discrete_event_simulation::run_single_thread::<AirTraffic>(
    //     vec![Airport::LAX, Airport::JFK, Airport::ORD]
    //         .into_iter()
    //         .collect(),
    //     NaiveDateTime::from_str("2000-01-02T00:00:00").unwrap(),
    // )
    // .unwrap()
}

struct AirTraffic;

impl discrete_event_simulation::Model for AirTraffic {
    type LogicalProcessId = Airport;
    type VirtualTime = DateTimeUtc;
    type State = LandingQueue;
    type Event = Event;
    type Output = Log;
    type Error = des::CausalityViolation;

    fn initialize(id: &Self::LogicalProcessId) -> (Self::State, Self::Event) {
        Default::default()
    }

    fn start_time() -> Self::VirtualTime {
        chrono::Utc::now()
    }

    fn process_event(
        scheduler: des::Scheduler<Self>,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        match scheduler.event() {
            AirportEvent::Init => todo!(),
            AirportEvent::Arrival(aircraft) => todo!(),
            AirportEvent::Landing => todo!(),
            AirportEvent::Departure(aircraft, airport) => todo!(),
        }
    }
}

type LandingQueue = VecDeque<Aircraft>;

#[derive(Clone, Copy)]
enum AirportEvent {
    Init,
    Arrival(Aircraft),
    Landing,
    Departure(Aircraft, Airport),
}

#[derive(Debug, Clone, Copy, Hash, Ord, PartialEq, Eq, PartialOrd)]
enum Airport {
    LAX,
    JFK,
    ORD,
}

#[derive(Clone, Copy)]
struct Aircraft {
    origin: Airport,
}

impl Aircraft {
    fn depart(self, origin: Airport) -> Self {
        Self { origin }
    }
}

struct Log;

impl des::Committable for Log {
    fn commit(self) {}
}
