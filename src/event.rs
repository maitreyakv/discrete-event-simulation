mod key;
mod queue;

use key::EventKey;

struct Event<Data, VirtualTime, LogicalProcessId>
where
    VirtualTime: Ord,
    LogicalProcessId: Ord,
{
    key: EventKey<VirtualTime, LogicalProcessId>,
    data: Data,
}

impl<Data, VirtualTime, LogicalProcessId> Ord for Event<Data, VirtualTime, LogicalProcessId>
where
    VirtualTime: Ord,
    LogicalProcessId: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<Data, VirtualTime, LogicalProcessId> PartialOrd for Event<Data, VirtualTime, LogicalProcessId>
where
    VirtualTime: Ord,
    LogicalProcessId: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.key.cmp(&other.key))
    }
}

impl<Data, VirtualTime, LogicalProcessId> Eq for Event<Data, VirtualTime, LogicalProcessId>
where
    VirtualTime: Ord,
    LogicalProcessId: Ord,
{
}

impl<Data, VirtualTime, LogicalProcessId> PartialEq for Event<Data, VirtualTime, LogicalProcessId>
where
    VirtualTime: Ord,
    LogicalProcessId: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
