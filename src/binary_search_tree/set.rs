//! a binary search tree with keys but no associated value
//! a degenerate form of `BsTreeMap`

use crate::entry::Entry;
use super::{map::BsTreeMap, node::Node};
pub struct BsTreeSet<T>
where
    T: Ord,
{
    map: BsTreeMap<T, ()>,
}

impl<K> BsTreeSet<K>
where
    K: Ord,
{
    pub fn new() -> BsTreeSet<K> {
        BsTreeSet {
            map: BsTreeMap::new(),
        }
    }

    /// inserts a new entry with key `key`
    /// if such an 
    pub fn insert(&mut self, key: K) {
        self.map.insert(key, ());
    }

    /// removes the entry with key `key`
    pub fn remove(&mut self, key: &K) {
        self.map.remove(key);
    }

    /// returns the number of entries in `self`
    pub fn size(&self) -> usize {
        self.map.size()
    }

    /// returns true if `self` contains an entry `key`
    /// otherwise returns false
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains(key)
    }

    pub fn extend(&mut self, other: BsTreeSet<K>) {
        self.map.merge(other.map);
    }

    pub fn smallest(&self) -> Option<&Entry<K, ()>> {
        self.map.smallest()
    }

    pub fn smallest_mut(&mut self) -> Option<&mut Entry<K, ()>> {
        self.map.smallest_mut()
    }

    pub fn largest(&self) -> Option<&Entry<K, ()>> {
        self.map.largest()
    }

    pub fn largest_mut(&mut self) -> Option<&mut Entry<K, ()>> {
        self.map.smallest_mut()
    }
}

impl<T> Default for BsTreeSet<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
