//! a binary search tree with keys but no associated value
//! a degenerate form of `BsTreeMap`

use super::map::BsTreeMap;
pub struct BsTreeSet<T>
where
    T: Ord,
{
    map: BsTreeMap<T, ()>,
}

impl<T> BsTreeSet<T>
where
    T: Ord,
{
    pub fn new() -> BsTreeSet<T> {
        BsTreeSet {
            map: BsTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: T) {
        self.map.insert(key, ());
    }

    pub fn remove(&mut self, key: T) {
        self.map.remove(key);
    }

    pub fn size(&self) -> usize {
        self.map.size()
    }

    pub fn contains(&self, key: T) -> bool {
        self.map.contains(key)
    }

    pub fn extend(&mut self, other: BsTreeSet<T>) {
        self.map.extend(other.map);
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
