use crate::Model;

pub(crate) struct EventKey<M>
where
    M: Model,
{
    time: M::VirtualTime,
    pub(crate) location: M::LogicalProcessId,
    age: usize,
    origin: M::LogicalProcessId,
    sequence_number: usize,
}

impl<M> EventKey<M>
where
    M: Model,
{
    fn create_first(
        time: M::VirtualTime,
        location: M::LogicalProcessId,
        origin: M::LogicalProcessId,
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
        time: M::VirtualTime,
        location: M::LogicalProcessId,
        origin: M::LogicalProcessId,
        sequence_number: usize,
    ) -> Self
    where
        M::VirtualTime: PartialEq,
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

    pub(crate) fn time(&self) -> &M::VirtualTime {
        &self.time
    }
}

impl<M> Ord for EventKey<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time
            .cmp(&other.time)
            .then(self.location.cmp(&other.location))
            .then(self.age.cmp(&other.age))
            .then(self.origin.cmp(&other.origin))
            .then(self.sequence_number.cmp(&other.sequence_number))
    }
}

impl<M> PartialOrd for EventKey<M>
where
    M: Model,
    M::VirtualTime: Ord,
    M::LogicalProcessId: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M> PartialEq for EventKey<M>
where
    M: Model,
    M::VirtualTime: PartialEq,
    M::LogicalProcessId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
            && self.location == other.location
            && self.age == other.age
            && self.origin == other.origin
            && self.sequence_number == other.sequence_number
    }
}

impl<M> Eq for EventKey<M>
where
    M: Model,
    M::VirtualTime: PartialEq,
    M::LogicalProcessId: PartialEq,
{
}

impl<M> Clone for EventKey<M>
where
    M: Model,
    M::VirtualTime: Clone,
    M::LogicalProcessId: Clone,
{
    fn clone(&self) -> Self {
        Self {
            time: self.time.clone(),
            location: self.location.clone(),
            age: self.age.clone(),
            origin: self.origin.clone(),
            sequence_number: self.sequence_number.clone(),
        }
    }
}
