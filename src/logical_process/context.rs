use crate::{Model, event::Event};

pub struct Context<'a, M>
where
    M: Model,
    M::LogicalProcessId: Ord,
{
    pub(crate) event: &'a Event<M>,
    pub(crate) state: &'a M::State,
}

impl<'a, M> Context<'a, M>
where
    M: Model,
    M::LogicalProcessId: Ord,
{
    pub fn id(&self) -> &M::LogicalProcessId {
        self.event.location()
    }

    pub fn time(&self) -> &M::VirtualTime {
        &self.event.key.time
    }

    pub fn event(&self) -> &M::Event {
        &self.event.data
    }

    pub fn state(&self) -> &M::State {
        self.state
    }
}
