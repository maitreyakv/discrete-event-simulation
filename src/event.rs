use std::cmp::Ordering;

use crate::Model;

struct Event<M: Model> {
    data: M::Event
}

// pub struct Event<E, T: Ord> {
//     data: E,
//     timestamp: T,
//     age: usize,
//     sender_id: usize,
//     sequence_number: usize,
// }
//
// impl<D, T: Ord> Ord for Event<D, T> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.timestamp.cmp(&other.timestamp)
//             .then(self.age.cmp(&other.age))
//             .then(self.sender_id.cmp(&other.sender_id))
//             .then(self.sequence_number.cmp(&other.sequence_number))
//     }
// }
//
// impl<D, T: Ord> PartialOrd for Event<D, T> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.timestamp.cmp(&other.timestamp)
//             .then(self.age.cmp(&other.age))
//             .then(self.sender_id.cmp(&other.sender_id))
//             .then(self.sequence_number.cmp(&other.sequence_number)))
//     }
// }
//
// impl<D, T: Ord> PartialEq for Event<D, T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.timestamp == other.timestamp 
//             && self.age == other.age 
//             && self.sender_id == other.sender_id 
//             && self.sequence_number == other.sequence_number
//     }
// }
//
// impl <D, T: Ord> Eq for Event<D, T> {}
