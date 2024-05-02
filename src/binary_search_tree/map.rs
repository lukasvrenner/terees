//! a binary search tree with keys and values
//! note: this tree makes no attempts to maintain balance

use super::entry::Entry;
pub struct BsTreeMap<K, V>
where
    K: Ord,
{
    head: Option<Box<Entry<K, V>>>,
    size: usize,
}

impl<K, V> BsTreeMap<K, V>
where
    K: Ord,
{
    /// creates an empty BsTree<T>
    #[inline]
    pub fn new() -> BsTreeMap<K, V> {
        BsTreeMap {
            head: None,
            size: 0,
        }
    }

    /// returns the number of entries in `self`
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// returns `true` if `self` contains an entry with key `key`
    /// otherwise returns `false`
    #[inline]
    pub fn contains(&self, key: K) -> bool {
        match self.head {
            Some(ref entry) => entry.contains(key),
            None => false,
        }
    }

    /// merges `other` with `self`
    pub fn extend(&mut self, other: BsTreeMap<K, V>) {
        self.size += other.size;
        todo!();
    }

    /// removes the entry with the given key
    pub fn remove(&mut self, key: K) {
        if let Some(ref mut entry) = self.head {
            self.size -= 1;
            todo!();
        }
    }

    /// sets the value of the key with key `key` to `value`
    /// if `key` already exists, the value is overridden
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        match self.head {
            Some(ref mut entry) => entry.insert(key, value),
            None => self.head = Some(Box::from(Entry::new(key, value))),
        }
        self.size += 1;
    }

    /// returns an optional reference to the `value` with key `key`
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.head {
            Some(ref entry) => entry.get(key),
            None => None,
        }
    }

    /// returns an optional mutable reference to the `value` with key `key`
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.head {
            Some(ref mut entry) => entry.get_mut(key),
            None => None,
        }
    }

    /// returns an optional reference to the smallest entry
    pub fn smallest(&self) -> Option<&Entry<K, V>> {
        match self.head {
            Some(ref entry) => Some(entry.smallest()),
            None => None,
        }
    }

    /// returns an optional mutable reference to the smallest entry
    pub fn smallest_mut(&mut self) -> Option<&mut Entry<K, V>> {
        match self.head {
            Some(ref mut entry) => Some(entry.smallest_mut()),
            None => None,
        }
    }

    /// returns an optional reference to the largest entry
    pub fn largest(&self) -> Option<&Entry<K, V>> {
        match self.head {
            Some(ref entry) => Some(entry.largest()),
            None => None,
        }
    }

    /// returns an optional mutable reference to the largest entry
    pub fn largest_mut(&mut self) -> Option<&mut Entry<K, V>> {
        match self.head {
            Some(ref mut entry) => Some(entry.largest_mut()),
            None => None,
        }
    }
}

impl<K, V> From<Entry<K, V>> for BsTreeMap<K, V>
where
    K: Ord,
{
    fn from(value: Entry<K, V>) -> Self {
        let size = value.size();
        BsTreeMap {
            head: Some(Box::from(value)),
            size,
        }
    }
}

impl<K, V> Default for BsTreeMap<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
