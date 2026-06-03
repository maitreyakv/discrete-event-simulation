use discrete_event_simulation::{self as des, Scheduler};

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

    fn initialize(_id: &Self::LogicalProcessId) -> (Self::State, Self::Event) {
        Default::default()
    }

    fn start_time() -> Self::VirtualTime {
        chrono::Utc::now()
    }

    fn process_event(
        mut scheduler: des::Scheduler<Self>,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        match scheduler.event() {
            Event::Init => {
                match *scheduler.logical_process_id() {
                    Airport::LAX => {
                        Self::schedule_departure(&mut scheduler, Airport::JFK, 5)?;
                        Self::schedule_departure(&mut scheduler, Airport::JFK, 15)?;
                    }
                    Airport::JFK => {
                        Self::schedule_departure(&mut scheduler, Airport::LAX, 10)?;
                        Self::schedule_departure(&mut scheduler, Airport::JFK, 20)?;
                    }
                    Airport::ORD => {
                        Self::schedule_departure(&mut scheduler, Airport::JFK, 15)?;
                        Self::schedule_departure(&mut scheduler, Airport::JFK, 25)?;
                    }
                };
                Ok((Default::default(), Log))
            }
            Event::Arrival(aircraft) => todo!(),
            Event::Landing => todo!(),
            Event::Departure(aircraft, airport) => todo!(),
        }
    }
}

impl AirTraffic {
    fn schedule_departure(
        scheduler: &mut des::Scheduler<Self>,
        destination: Airport,
        minutes: i64,
    ) -> Result<(), des::CausalityViolation> {
        todo!()
    }
}

type LandingQueue = VecDeque<Aircraft>;

type DateTimeUtc = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Copy, Default)]
enum Event {
    #[default]
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
    fn depart(origin: Airport) -> Self {
        Self { origin }
    }
}

struct Log;

impl des::Committable for Log {
    fn commit(self) {}
}
