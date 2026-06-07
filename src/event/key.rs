#[derive(Ord, PartialOrd, PartialEq, Eq)]
pub(crate) struct EventKey<VirtualTime, LogicalProcessId>
where
    VirtualTime: Ord,
    LogicalProcessId: Ord,
{
    time: VirtualTime,
    location: LogicalProcessId,
    age: usize,
    origin: LogicalProcessId,
    sequence_number: usize,
}

impl<VirtualTime, LogicalProcessId> EventKey<VirtualTime, LogicalProcessId>
where
    VirtualTime: Ord,
    LogicalProcessId: Ord,
{
    fn create_first(
        time: VirtualTime,
        location: LogicalProcessId,
        origin: LogicalProcessId,
    ) -> Self {
        Self {
            time,
            location,
            age: 0,
            origin,
            sequence_number: 0,
        }
    }

    fn create_another(
        &self,
        time: VirtualTime,
        location: LogicalProcessId,
        origin: LogicalProcessId,
        sequence_number: usize,
    ) -> Self
    where
        VirtualTime: PartialEq,
    {
        let age = if self.time == time { self.age + 1 } else { 0 };
        Self {
            time,
            location,
            age,
            origin,
            sequence_number,
        }
    }
}
