use discrete_event_simulation::{self as des};

use std::collections::VecDeque;

fn main() {
    des::run_single_thread::<AirTraffic>(
        vec![Airport::LAX, Airport::JFK, Airport::ORD]
            .into_iter()
            .collect(),
        chrono::Utc::now(),
    )
    .unwrap()
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
        match *scheduler.event() {
            Event::Init => {
                match *scheduler.logical_process_id() {
                    Airport::LAX => {
                        Self::depart_aircraft(&mut scheduler, Airport::JFK)?;
                        Self::depart_aircraft(&mut scheduler, Airport::ORD)?;
                    }
                    Airport::JFK => {
                        Self::depart_aircraft(&mut scheduler, Airport::LAX)?;
                        Self::depart_aircraft(&mut scheduler, Airport::ORD)?;
                    }
                    Airport::ORD => {
                        Self::depart_aircraft(&mut scheduler, Airport::JFK)?;
                        Self::depart_aircraft(&mut scheduler, Airport::LAX)?;
                    }
                };
                Ok((Default::default(), Log))
            }
            Event::Arrival(aircraft) => {
                Self::arrive_aircraft(&mut scheduler, aircraft).map(|lq| (lq, Log))
            }
            Event::Landing => Self::land_aircraft(&mut scheduler).map(|lq| (lq, Log)),
            Event::Departure(destination) => {
                Self::depart_aircraft(&mut scheduler, destination)?;
                Ok((scheduler.state().clone(), Log))
            }
        }
    }
}

impl AirTraffic {
    fn depart_aircraft(
        scheduler: &mut des::Scheduler<Self>,
        destination: Airport,
    ) -> Result<(), des::CausalityViolation> {
        let this_airport = *scheduler.logical_process_id();
        scheduler.schedule_event(
            Event::Arrival(Aircraft::depart(this_airport)),
            *scheduler.time() + this_airport.time_to(&destination),
            destination,
        )
    }

    fn land_aircraft(
        scheduler: &mut des::Scheduler<Self>,
    ) -> Result<LandingQueue, des::CausalityViolation> {
        let mut landing_queue = scheduler.state().clone();
        if let Some(aircraft) = landing_queue.pop_front() {
            scheduler.schedule_internal_event(
                Event::Departure(aircraft.origin),
                *scheduler.time() + chrono::TimeDelta::minutes(60),
            )?;
        }
        if !landing_queue.is_empty() {
            scheduler.schedule_internal_event(
                Event::Landing,
                *scheduler.time() + chrono::TimeDelta::minutes(15),
            )?;
        }
        Ok(landing_queue)
    }

    fn arrive_aircraft(
        scheduler: &mut des::Scheduler<Self>,
        aircraft: Aircraft,
    ) -> Result<LandingQueue, des::CausalityViolation> {
        let mut landing_queue = scheduler.state().clone();
        if landing_queue.is_empty() {
            scheduler.schedule_internal_event(
                Event::Landing,
                *scheduler.time() + chrono::TimeDelta::minutes(15),
            )?;
        }
        landing_queue.push_back(aircraft);
        Ok(landing_queue)
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
    Departure(Airport),
}

#[derive(Debug, Clone, Copy, Hash, Ord, PartialEq, Eq, PartialOrd)]
enum Airport {
    LAX,
    JFK,
    ORD,
}

impl Airport {
    fn time_to(&self, other: &Airport) -> chrono::TimeDelta {
        match (self, other) {
            (Airport::LAX, Airport::JFK) => chrono::TimeDelta::minutes(270),
            (Airport::JFK, Airport::LAX) => chrono::TimeDelta::minutes(270),
            (Airport::LAX, Airport::ORD) => chrono::TimeDelta::minutes(190),
            (Airport::ORD, Airport::LAX) => chrono::TimeDelta::minutes(190),
            (Airport::JFK, Airport::ORD) => chrono::TimeDelta::minutes(80),
            (Airport::ORD, Airport::JFK) => chrono::TimeDelta::minutes(80),
            _ => chrono::TimeDelta::zero(),
        }
    }
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
