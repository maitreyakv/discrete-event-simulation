use std::collections::BTreeMap;

use crate::Committable;
use crate::Model;
use crate::event::AntiEvent;
use crate::event::Event;
use crate::event::EventKey;

pub(crate) struct History<M>(BTreeMap<EventKey<M>, Record<M>>)
where
    M: Model;

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
        self.0.insert(
            event.key,
            Record {
                event_data: event.data,
                prior_state,
                output,
                anti_events,
            },
        );
    }

    pub(crate) fn contains_event(&self, key: &EventKey<M>) -> bool {
        self.0.contains_key(key)
    }

    pub(crate) fn rollback(&mut self, until: &EventKey<M>) -> impl Iterator<Item = Rollback<M>> {
        std::iter::from_fn(move || {
            if let Some((key, _)) = self.0.last_key_value()
                && key >= until
            {
                self.0.pop_last().map(|(key, record)| Rollback {
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

    pub(crate) fn collect_fossils(&mut self, global_virtual_time: &M::VirtualTime)
    where
        M::Output: Committable,
    {
        while let Some((key, _)) = self.0.first_key_value()
            && key.time() < global_virtual_time
        {
            self.0.pop_first().unwrap().1.output.commit();
        }
    }
}

impl<M> Default for History<M>
where
    M: Model,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

pub(crate) struct Rollback<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    pub(crate) event: Event<M>,
    pub(crate) prior_state: M::State,
    pub(crate) anti_events: Vec<AntiEvent<M>>,
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
