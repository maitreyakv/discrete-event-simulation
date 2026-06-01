use std::collections::HashSet;

use crate::{logical_process::LogicalProcessSet, Model};

pub fn run_single_thread<M: Model>(ids: HashSet<M::LogicalProcessId>, steps: usize) {
    let mut worker: Worker<M> = Worker {
        logical_processes: LogicalProcessSet::from_ids(ids),
    };
    for _ in 0..steps {
        worker.step();
    }
}

struct Worker<M: Model> {
    logical_processes: LogicalProcessSet<M>,
}

impl<M: Model> Worker<M> {
    fn step(&mut self) {
        self.logical_processes.process_next_event();
    }
}
