//! a binary search tree with keys and values
//! note: this tree makes no attempts to maintain balance

use super::node::Node;
use crate::entry::Entry;
pub struct BsTreeMap<K, V>
where
    K: Ord,
{
    head: Option<Box<Node<K, V>>>,
    size: usize,
}

impl<K, V> BsTreeMap<K, V>
where
    K: Ord,
{
    /// creates an empty BsTree<T>
    #[inline]
    pub const fn new() -> BsTreeMap<K, V> {
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

    /// returns `true` if `self` contains an node with key `key`
    /// otherwise returns `false`
    #[inline]
    pub fn contains(&self, key: &K) -> bool {
        match self.head {
            Some(ref node) => node.contains(key),
            None => false,
        }
    }

    /// merges `other` with `self`
    pub fn merge(&mut self, other: BsTreeMap<K, V>) {
        self.size += other.size;
        todo!();
    }

    /// removes the node with the given key
    pub fn remove(&mut self, key: &K) {
        if let Some(ref mut node) = self.head {
            self.size -= 1;
            todo!();
        }
    }

    /// sets the value of the key with key `key` to `value`
    /// if `key` already exists, the value is overridden
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        match self.head {
            Some(ref mut node) => {
                if node.insert(key, value) {
                    self.size += 1;
                }
            }
            None => {
                self.head = Some(Box::from(Node::new(key, value)));
                self.size += 1;
            }
        }
    }

    /// returns an optional reference to the `value` with key `key`
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.head {
            Some(ref node) => node.get(key),
            None => None,
        }
    }

    /// returns an optional mutable reference to the `value` with key `key`
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.head {
            Some(ref mut node) => node.get_mut(key),
            None => None,
        }
    }

    /// returns an optional reference to the smallest node
    pub fn smallest(&self) -> Option<&Entry<K, V>> {
        match self.head {
            Some(ref node) => Some(node.smallest()),
            None => None,
        }
    }

    /// returns an optional mutable reference to the smallest node
    pub fn smallest_mut(&mut self) -> Option<&mut Entry<K, V>> {
        match self.head {
            Some(ref mut node) => Some(node.smallest_mut()),
            None => None,
        }
    }

    /// returns an optional reference to the largest node
    pub fn largest(&self) -> Option<&Entry<K, V>> {
        match self.head {
            Some(ref node) => Some(node.largest()),
            None => None,
        }
    }

    /// returns an optional mutable reference to the largest node
    pub fn largest_mut(&mut self) -> Option<&mut Entry<K, V>> {
        match self.head {
            Some(ref mut node) => Some(node.largest_mut()),
            None => None,
        }
    }

    pub fn node(&self, key: &K) -> Option<&Entry<K, V>> {
        match self.head {
            Some(ref node) => node.entry(key),
            None => None,
        }
    }

    pub fn node_mut(&mut self, key: &K) -> Option<&mut Entry<K, V>> {
        match self.head {
            Some(ref mut node) => node.entry_mut(key),
            None => None,
        }
    }
}

impl<K, V> From<Node<K, V>> for BsTreeMap<K, V>
where
    K: Ord,
{
    fn from(value: Node<K, V>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_tree() -> BsTreeMap<usize, String> {
        let mut tree = BsTreeMap::new();
        tree.insert(5, " , ".to_string());
        tree.insert(3, "hello".to_string());
        tree.insert(6, "world".to_string());
        tree.insert(7, "this is a string!".to_string());
        tree.insert(1, "this is value has a key of 1".to_string());
        tree.insert(4, "hmmm".to_string());

        tree
    }

    #[test]
    fn insert() {
        let mut tree = BsTreeMap::new();
        tree.insert(5, " , ");
        tree.insert(3, "hello");
        tree.insert(7, "haha get replaced");
        tree.insert(6, "world");
        tree.insert(7, "hii");

        assert_eq!(tree.size(), 4);
        assert_eq!(tree.get(&7), Some(&"hii"));
        assert_eq!(tree.get(&8), None);
    }

    #[test]
    fn contains() {}
}
