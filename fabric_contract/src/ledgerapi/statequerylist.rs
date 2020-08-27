/*
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::ledgerapi::state::*;
use std::iter::FromIterator;

//
pub struct StateQueryList {
    states: Vec<State>,
}

impl StateQueryList {
    pub fn new(states: Vec<State>) -> Self {
        Self { states }
    }

    // pub fn iter(& self) -> IterHelper{
    //     self.into_iter()
    // }

    // pub fn iter_mut(&mut self) -> IterMutHelper{
    //     self.into_iter()
    // }    

}

// structure helper for consuming iterator.
pub struct IntoIteratorHelper {
    iter: ::std::vec::IntoIter<State>,
}

impl IntoIterator for StateQueryList {
    type Item = State;
    type IntoIter = IntoIteratorHelper;

    fn into_iter(self) -> Self::IntoIter {
        IntoIteratorHelper {
            iter: self.states.into_iter(),
        }
    }
}

// now, implements Iterator trait for the helper struct, to be used by adapters
impl Iterator for IntoIteratorHelper {
    type Item = State;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
    }
}

// // ---
// // structure helper for non-consuming iterator.
// pub struct IterHelper {
//     iter: ::std::slice::Iter<State>,
// }

// // implement the IntoIterator trait for a non-consuming iterator. Iteration will
// // borrow the Words structure 
// impl IntoIterator for StateQueryList {
//     type Item = State;
//     type IntoIter = IterHelper;

//     // note that into_iter() is consuming self
//     fn into_iter(self) -> Self::IntoIter {
//         IterHelper {
//             iter: self.states.iter(),
//         }
//     }
// }

// // now, implements Iterator trait for the helper struct, to be used by adapters
// impl Iterator for IterHelper {
//     type Item = State;

//     // just return the str reference
//     fn next(&mut self) -> Option<Self::Item> {
//             self.iter.next()
//     }
// }

// // structure helper for mutable non-consuming iterator.
// pub struct IterMutHelper {
//     iter: ::std::slice::IterMut<State>,
// }

// // implement the IntoIterator trait for a mutable non-consuming iterator. Iteration will
// // mutably borrow the Words structure 
// impl IntoIterator for &mut StateQueryList{
//     type Item = mut State;
//     type IntoIter = IterMutHelper;

//     // note that into_iter() is consuming self
//     fn into_iter(self) -> Self::IntoIter {
//         IterMutHelper {
//             iter: self.states.iter_mut(),
//         }
//     }
// }

// // now, implements Iterator trait for the helper struct, to be used by adapters
// impl Iterator for IterMutHelper {
//     type Item = mut State;

//     // just return the str reference
//     fn next(&mut self) -> Option<Self::Item> {
//             self.iter.next()
//     }
// }

// implement FromIterator
impl FromIterator<State> for StateQueryList {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = State> {

        // create and return Words structure
        StateQueryList {
            states: iter.into_iter().collect(),
        }
    }
}