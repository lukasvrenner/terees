//! a generic binary search tree
//! this tree makes no attempts to maintain balance

use std::cmp::Ordering;
use crate::binary_search_tree::entry::Entry;
pub struct BsTreeMap<T, U> where T: Ord {
    head: Option<Box<Entry<T, U>>>,
    size: usize,
}

impl<T, U> BsTreeMap<T, U> where T: Ord {

    /// creates an empty BsTree<T>
    #[inline]
    pub fn new() -> BsTreeMap<T, U> {
        BsTreeMap { head: None, size: 0 }
    }

    /// returns the number of entries in `self`
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// returns `true` if `self` contains an entry with key `key`
    /// otherwise returns `false`
    #[inline]
    pub fn contains(&self, key: T) -> bool {
        match self.head {
            Some(ref entry) => entry.contains(key),
            None => false,
        }
    }

    /// merges `other` with `self`
    pub fn extend(&mut self, other: BsTreeMap<T, U>) {
        self.size += other.size;
        todo!();
    }

    /// removes the entry with the given key
    pub fn remove(&mut self, key: T) {
        if let Some(ref mut entry) = self.head {
            self.size -= 1;
            todo!();
        }
    }

    /// inserts a new entry with key `key` and value `value`
    #[inline]
    pub fn insert(&mut self, key: T, value: U) {
        match self.head {
            Some(ref mut entry) => entry.add(key, value),
            None => self.head = Some(Box::from(Entry::new(key, value))),
        }
        self.size += 1;
    }

    /// returns an optional reference to the `value` with key `key`
    #[inline]
    pub fn get(&self, key: T) -> Option<&U> {
        match self.head {
            Some(ref entry) => entry.get(key),
            None => None,
        }
    }

    /// returns an optional mutable reference to the `value` with key `key`
    pub fn get_mut(&mut self, key: T) -> Option<&mut U> {
        match self.head {
            Some(ref mut entry) => entry.get_mut(key),
            None => None,
        }
    }

}

impl<T, U> From<Entry<T, U>> for BsTreeMap<T, U> where T: Ord {
    fn from(value: Entry<T, U>) -> Self {
        let size = value.size();
        BsTreeMap { head: Some(Box::from(value)), size, }
    }
}

