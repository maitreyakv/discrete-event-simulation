use std::collections::BTreeMap;

use crate::Committable;
use crate::Model;
use crate::event::AntiEvent;
use crate::event::Event;
use crate::event::EventKey;

#[derive(Default)]
pub(crate) struct History<M>
where
    M: Model,
{
    records: BTreeMap<EventKey<M>, Record<M>>,
}

impl<M> History<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    pub(crate) fn save_event(
        &mut self,
        event: Event<M>,
        prior_state: M::State,
        output: M::Output,
        anti_events: Vec<AntiEvent<M>>,
    ) {
        self.records.insert(
            event.key,
            Record {
                event_data: event.data,
                prior_state,
                output,
                anti_events,
            },
        );
    }

    fn contains_event(&self, key: &EventKey<M>) -> bool {
        self.records.contains_key(key)
    }

    fn rollback(&mut self, until: &EventKey<M>) -> impl Iterator<Item = Rollback<M>> {
        std::iter::from_fn(move || {
            if let Some((key, _)) = self.records.last_key_value()
                && key >= until
            {
                self.records.pop_last().map(|(key, record)| Rollback {
                    event: Event {
                        key,
                        data: record.event_data,
                    },
                    prior_state: record.prior_state,
                    anti_events: record.anti_events,
                })
            } else {
                None
            }
        })
    }

    fn collect_fossils(&mut self, global_virtual_time: &M::VirtualTime)
    where
        M::Output: Committable,
    {
        while let Some((key, _)) = self.records.first_key_value()
            && key.time() < global_virtual_time
        {
            self.records.pop_first().unwrap().1.output.commit();
        }
    }
}

struct Rollback<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    event: Event<M>,
    prior_state: M::State,
    anti_events: Vec<AntiEvent<M>>,
}

struct Record<M>
where
    M: Model,
{
    event_data: M::Event,
    prior_state: M::State,
    output: M::Output,
    anti_events: Vec<AntiEvent<M>>,
}
