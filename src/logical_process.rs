mod history;

use history::History;

use crate::Model;

pub(crate) struct LogicalProcess<M>
where
    M: Model,
{
    pub(crate) id: M::LogicalProcessId,
    pub(crate) state: M::State,
    pub(crate) sequence_number: usize,
    pub(crate) history: History<M>,
}
