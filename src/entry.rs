//! an entry structure for maps
pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Entry<K, V> {
    pub const fn new(key: K, value: V) -> Entry<K, V> {
        Entry { key, value }
    }

    /// returns a reference to the entry's key
    pub fn key(&self) -> &K {
        &self.key
    }

    /// returns a reference to the entry's value
    pub fn value(&self) -> &V {
        &self.value
    }

    /// returns a mutable reference to the entry's value
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}
