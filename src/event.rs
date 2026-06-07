mod anti;
mod key;
mod queue;

use crate::Model;
use anti::AntiEvent;
use key::EventKey;

struct Event<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    key: EventKey<M>,
    data: M::Event,
}

impl<M> Ord for Event<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<M> PartialOrd for Event<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M> Eq for Event<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
}

impl<M> PartialEq for Event<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
