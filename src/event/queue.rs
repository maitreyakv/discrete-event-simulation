use std::collections::BTreeSet;

use super::Event;

// pub(crate) struct EventQueue<Data, VirtualTime, LogicalProcessId>(
//     BTreeSet<Event<Data, VirtualTime, LogicalProcessId>>,
// );
//
// impl<Data, VirtualTime, LogicalProcessId> EventQueue<Data, VirtualTime, LogicalProcessId> {
//     pub(crate) fn insert(&mut self, event: Event<Data, VirtualTime, LogicalProcessId>)
//     where
//         VirtualTime: Ord,
//     {
//         self.0.insert(event);
//     }
// }
