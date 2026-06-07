struct Event<Data, VirtualTime, LogicalProcessId> {
    key: EventKey<VirtualTime, LogicalProcessId>,
    data: Data,
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct EventKey<VirtualTime, LogicalProcessId> {
    time: VirtualTime,
    location: LogicalProcessId,
    age: usize,
    origin: LogicalProcessId,
    sequence_number: usize,
}

impl<VirtualTime, LogicalProcessId> EventKey<VirtualTime, LogicalProcessId> {
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
