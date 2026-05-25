use crate::{Model, logical_process::LogicalProcess};
use std::collections::HashMap;

pub struct SingleThreadWorker<M: Model> {
    logical_processes: HashMap<usize, LogicalProcess<M>>,
}

impl<M: Model> SingleThreadWorker<M> {
    pub fn new() -> Self {
        Self {
            logical_processes: HashMap::default(),
        }
    }

    pub fn with_logical_process(&mut self, id: usize) -> &mut Self {
        self.logical_processes.insert(id, LogicalProcess::new(id));
        self
    }

    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            if let Some((next_logical_process_id, _)) = self
                .logical_processes
                .iter()
                .filter_map(|(id, lp)| lp.next_event().map(|e| (id, e.timestamp)))
                .min_by_key(|(_, ts)| *ts)
            {
                let id = next_logical_process_id.to_owned();
                self.logical_processes
                    .get_mut(&id)
                    .unwrap()
                    .process_next_event();
            }
        }
    }
}
