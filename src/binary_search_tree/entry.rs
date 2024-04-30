//! an entry in `BsTreeMap`
use std::cmp::Ordering;
/// `left` represents entrys that have smaller `key`s than `self.key`
/// `right` represents entrys that have `key`s greater than or equal to `self.key`
pub struct Entry<T, U> where T: Ord {
    key: T,
    value: U,
    left: Option<Box<Entry<T, U>>>,
    right: Option<Box<Entry<T, U>>>,
}

impl<T, U> Entry<T, U> where T: Ord {
    /// creates a single `Entry` with key `key` and value `value`
    #[inline]
    pub fn new(key: T, value: U) -> Entry<T, U> {
        Entry {
            key,
            value,
            left: None,
            right: None,
        }
    }

    /// returns an optional reference to the `value` with key `key`
    pub fn get(&self, key: T) -> Option<&U> {
        match key.cmp(&self.key) {
            Ordering::Less => {
                match self.left {
                    Some(ref entry) => entry.get(key),
                    None => None,
                }
            },
            Ordering::Greater => {
                match self.right {
                    Some(ref entry) => entry.get(key),
                    None => None,
                }
            },
            Ordering::Equal => Some(&self.value),
        }
    }

    /// returns an optional mutable reference to the `value` with key `key`
    pub fn get_mut(&mut self, key: T) -> Option<&mut U> {
        match key.cmp(&mut self.key) {
            Ordering::Less => {
                match self.left {
                    Some(ref mut entry) => entry.get_mut(key),
                    None => None,
                }
            },
            Ordering::Greater => {
                match self.right {
                    Some(ref mut entry) => entry.get_mut(key),
                    None => None,
                }
            },
            Ordering::Equal => Some(&mut self.value),
        }
    }

    pub fn add(&mut self, key: T, value: U) {
        // check left branch
        if self.key < key {
            match &mut self.left {
                Some(ref mut tree) => tree.add(key, value),
                None => {
                    self.left = Some(Box::new(Entry::new(key, value)));
                }
            }
        } else {
            // right branch
            match &mut self.right {
                Some(ref mut tree) => tree.add(key, value),
                None => {
                    self.right = Some(Box::new(Entry::new(key, value)));
                }
            }
        }
    }

    pub fn remove(&mut self, key: T) {
        todo!();
    }

    pub fn find(&self, key: T) -> Option<&Entry<T, U>> {
        if self.key < key {
            match &self.left {
                Some(entry) => return entry.find(key),
                None => return None,
            }
        }
        if self.key > key {
            match &self.right {
                Some(entry) => return entry.find(key),
                None => return None,
            }
        }
        Some(self)
    }

    pub fn find_mut(&mut self, key: T) -> Option<&mut Entry<T, U>> {
        if self.key < key {
            match &mut self.left {
                Some(entry) => return entry.find_mut(key),
                None => return None,
            }
        }
        if self.key > key {
            match &mut self.right {
                Some(entry) => return entry.find_mut(key),
                None => return None,
            }
        }
        Some(self)
    }

    pub fn concat(&mut self, other: Entry<T, U>) {
        todo!()
    }

    pub fn contains(&self, key: T) -> bool {
        if self.key < key {
            match &self.left {
                Some(entry) => return entry.contains(key),
                None => return false,
            };
        }
        if self.key > key {
            match &self.right {
                Some(entry) => return entry.contains(key),
                None => return false,
            };
        }
        true
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
