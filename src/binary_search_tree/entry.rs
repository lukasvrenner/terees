//! an entry in `BsTreeMap`
use std::cmp::Ordering;
/// `left` represents entrys that have smaller `key`s than `self.key`
/// `right` represents entrys that have greater `key`s than `self.key`
pub struct Entry<K, V>
where
    K: Ord,
{
    key: K,
    value: V,
    left: Option<Box<Entry<K, V>>>,
    right: Option<Box<Entry<K, V>>>,
}

impl<K, V> Entry<K, V>
where
    K: Ord,
{
    /// creates a single `Entry` with key `key` and value `value`
    #[inline]
    pub fn new(key: K, value: V) -> Entry<K, V> {
        Entry {
            key,
            value,
            left: None,
            right: None,
        }
    }

    /// returns an optional reference to the `value` with key `key`
    pub fn get(&self, key: K) -> Option<&V> {
        match key.cmp(&self.key) {
            Ordering::Less => match self.left {
                Some(ref entry) => entry.get(key),
                None => None,
            },
            Ordering::Greater => match self.right {
                Some(ref entry) => entry.get(key),
                None => None,
            },
            Ordering::Equal => Some(&self.value),
        }
    }

    /// returns an optional mutable reference to the `value` with key `key`
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        match key.cmp(&mut self.key) {
            Ordering::Less => match self.left {
                Some(ref mut entry) => entry.get_mut(key),
                None => None,
            },
            Ordering::Greater => match self.right {
                Some(ref mut entry) => entry.get_mut(key),
                None => None,
            },
            Ordering::Equal => Some(&mut self.value),
        }
    }

    /// sets the value of the key with key `key` to `value`
    /// if `key` already exists, the value is overridden
    pub fn insert(&mut self, key: K, value: V) {
        match key.cmp(&self.key) {
            Ordering::Less => match self.left {
                Some(ref mut entry) => entry.insert(key, value),
                None => self.left = Some(Box::from(Entry::new(key, value))),
            },
            Ordering::Greater => match self.right {
                Some(ref mut entry) => entry.insert(key, value),
                None => self.right = Some(Box::from(Entry::new(key, value))),
            },
            Ordering::Equal => self.value = value,
        }
    }

    pub fn remove(&mut self, key: K) {
        todo!();
    }

    pub fn find(&self, key: K) -> Option<&Entry<K, V>> {
        match key.cmp(&self.key) {
            Ordering::Less => match self.left {
                Some(ref entry) => entry.find(key),
                None => None,
            },
            Ordering::Greater => match self.right {
                Some(ref entry) => entry.find(key),
                None => None,
            },
            Ordering::Equal => Some(self),
        }
    }

    pub fn find_mut(&mut self, key: K) -> Option<&mut Entry<K, V>> {
        match key.cmp(&self.key) {
            Ordering::Less => match self.left {
                Some(ref mut entry) => entry.find_mut(key),
                None => None,
            },
            Ordering::Greater => match self.right {
                Some(ref mut entry) => entry.find_mut(key),
                None => None,
            },
            Ordering::Equal => Some(self),
        }
    }

    pub fn concat(&mut self, other: Entry<K, V>) {
        todo!()
    }

    pub fn contains(&self, key: K) -> bool {
        match key.cmp(&self.key) {
            Ordering::Less => match self.left {
                Some(ref entry) => entry.contains(key),
                None => false,
            },
            Ordering::Greater => match self.right {
                Some(ref entry) => entry.contains(key),
                None => false,
            },
            Ordering::Equal => true,
        }
    }

    pub fn size(&self) -> usize {
        let mut size = 1;
        if let Some(ref entry) = self.right {
            size += entry.size();
        }
        if let Some(ref entry) = self.left {
            size += entry.size();
        }
        size
    }
}
