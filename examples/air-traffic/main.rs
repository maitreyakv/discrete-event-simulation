use std::{collections::VecDeque, str::FromStr};

use chrono::NaiveDateTime;

fn main() {
    discrete_event_simulation::run_single_thread::<AirTraffic>(
        vec![Airport::LAX, Airport::JFK, Airport::ORD]
            .into_iter()
            .collect(),
        NaiveDateTime::from_str("2000-01-02T00:00:00").unwrap(),
    )
    .unwrap()
}

struct AirTraffic;

impl discrete_event_simulation::Model for AirTraffic {
    type LogicalProcessId = Airport;
    type VirtualTime = NaiveDateTime;
    type State = AirportState;
    type Event = AirportEvent;
    type Output = Log;
    type Error = std::convert::Infallible;

    fn initialize(id: &Self::LogicalProcessId) -> (Self::State, Self::Event) {
        let state = AirportState {
            traffic_pattern: Default::default(),
        };
        let event = AirportEvent::Init(match id {
            Airport::LAX => vec![
                (
                    NaiveDateTime::from_str("2000-01-01T05:15:00").unwrap(),
                    Airport::JFK,
                ),
                (
                    NaiveDateTime::from_str("2000-01-01T10:20:00").unwrap(),
                    Airport::ORD,
                ),
            ],
            Airport::JFK => vec![
                (
                    NaiveDateTime::from_str("2000-01-01T06:30:00").unwrap(),
                    Airport::LAX,
                ),
                (
                    NaiveDateTime::from_str("2000-01-01T8:45:00").unwrap(),
                    Airport::ORD,
                ),
            ],
            Airport::ORD => vec![
                (
                    NaiveDateTime::from_str("2000-01-01T02:25:00").unwrap(),
                    Airport::LAX,
                ),
                (
                    NaiveDateTime::from_str("2000-01-01T4:50:00").unwrap(),
                    Airport::JFK,
                ),
            ],
        });
        (state, event)
    }

    fn start_time() -> Self::VirtualTime {
        NaiveDateTime::from_str("2000-01-01T00:00:00").unwrap()
    }

    fn process_event(
        scheduler: &mut discrete_event_simulation::Scheduler<Self>,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        match scheduler.event() {
            AirportEvent::Init(departures) => {
                departures.iter().for_each(|(departs_at, departs_to)| {
                    scheduler.schedule_event(
                        AirportEvent::Departure(*departs_to),
                        *departs_at,
                        *scheduler.logical_process_id(),
                    );
                });
                todo!()
            }
            AirportEvent::Arrival(aircraft) => todo!(),
            AirportEvent::Landing => todo!(),
            AirportEvent::Departure(destination) => todo!(),
        }
    }
}

struct AirportState {
    traffic_pattern: VecDeque<Aircraft>,
}

enum AirportEvent {
    Init(Vec<(NaiveDateTime, Airport)>),
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

struct Aircraft {
    departed: Airport,
}

struct Log;

impl discrete_event_simulation::Committable for Log {
    fn commit(self) {}
}
