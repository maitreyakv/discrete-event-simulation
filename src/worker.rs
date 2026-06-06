use std::collections::HashSet;

use crate::{DesError, Model, logical_process::LogicalProcessSet};

pub fn run_single_thread<M: Model>(
    ids: HashSet<M::LogicalProcessId>,
    until: M::VirtualTime,
) -> Result<(), DesError<M::Error>> {
    let mut logical_processes = LogicalProcessSet::<M>::from_ids(ids);
    while let Some(time) = logical_processes.time_of_next_event().cloned()
        && time < until
    {
        logical_processes.process_next_event()?;
        logical_processes.collect_fossils(&time);
    }
    Ok(())
}
