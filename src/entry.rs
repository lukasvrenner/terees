//! an entry structure for maps

#[derive(Debug, PartialEq)]
pub struct Entry<K, V: ?Sized> {
    key: K,
    pub value: Box<V>,
}

impl<K, V> Entry<K, V> {
    pub fn new(key: K, value: V) -> Entry<K, V> {
        Entry {
            key,
            value: Box::from(value),
        }
    }

    /// returns a reference to the entry's key
    pub fn key(&self) -> &K {
        &self.key
    }
}
