//! an node in `BsTreeMap`
use super::RemoveableNode;
use crate::entry::Entry;
use std::cmp::Ordering;

/// `left` represents entrys that have smaller `key`s than `self.key`
/// `right` represents entrys that have greater `key`s than `self.key`
pub struct Node<K, V>
where
    K: Ord,
{
    entry: Entry<K, V>,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

#[derive(Debug)]
pub enum RemoveError {
    DoesNotExist,
    RemoveMeIsRoot,
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    /// creates a single `Entry` with key `key` and value `value`
    #[inline]
    pub fn new(key: K, value: V) -> Node<K, V> {
        Node {
            entry: Entry::new(key, value),
            left: None,
            right: None,
        }
    }

    /// returns an optional reference to the `value` with key `key`
    pub fn get(&self, key: &K) -> Option<&V> {
        match key.cmp(self.entry.key()) {
            Ordering::Less => match self.left {
                Some(ref node) => node.get(key),
                None => None,
            },
            Ordering::Greater => match self.right {
                Some(ref node) => node.get(key),
                None => None,
            },
            Ordering::Equal => Some(self.entry.value()),
        }
    }

    /// returns an optional reference to the right node
    pub fn left(&self) -> Option<&Node<K, V>> {
        self.left.as_deref()
    }

    /// returns an optional mutable reference to the left node
    pub fn left_mut(&mut self) -> Option<&mut Node<K, V>> {
        self.left.as_deref_mut()
    }

    /// returns an optional reference to right node
    pub fn right(&self) -> Option<&Node<K, V>> {
        self.right.as_deref()
    }

    /// returns an optional mutable reference to the right node
    pub fn right_mut(&mut self) -> Option<&mut Node<K, V>> {
        self.right.as_deref_mut()
    }

    /// returns a reference to the key
    pub fn key(&self) -> &K {
        self.entry.key()
    }

    /// returns a reference to the value
    pub fn value(&self) -> &V {
        self.entry.value()
    }

    /// returns a mutable reference to the value
    pub fn value_mut(&mut self) -> &mut V {
        self.entry.value_mut()
    }

    /// returns an optional mutable reference to the `value` with key `key`
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match key.cmp(self.entry.key()) {
            Ordering::Less => match self.left {
                Some(ref mut node) => node.get_mut(key),
                None => None,
            },
            Ordering::Greater => match self.right {
                Some(ref mut node) => node.get_mut(key),
                None => None,
            },
            Ordering::Equal => Some(self.entry.value_mut()),
        }
    }

    /// sets the value of the entry with key `key` to `value`
    /// if `key` already exists, the value is overwritten
    /// otherwise, a new entry is added
    /// returns `true` if a *new* entry is added (not overwritten)
    /// else returns `false`
    pub fn insert(&mut self, key: K, value: V) -> bool {
        match key.cmp(self.entry.key()) {
            Ordering::Less => match self.left {
                Some(ref mut node) => node.insert(key, value),
                None => {
                    self.left = Some(Box::from(Node::new(key, value)));
                    true
                }
            },
            Ordering::Greater => match self.right {
                Some(ref mut node) => node.insert(key, value),
                None => {
                    self.right = Some(Box::from(Node::new(key, value)));
                    true
                }
            },
            Ordering::Equal => {
                *self.entry.value_mut() = value;
                false
            }
        }
    }

    /// inserts a new entry with key and value `key`, `value`
    /// if an entry with `key` already exists, does not overwrite old value
    /// returns `true` if a new entry is created
    /// else returns false
    pub fn try_insert(&mut self, key: K, value: V) -> bool {
        match key.cmp(self.entry.key()) {
            Ordering::Less => match self.left {
                Some(ref mut node) => node.insert(key, value),
                None => {
                    self.left = Some(Box::from(Node::new(key, value)));
                    true
                }
            },
            Ordering::Greater => match self.right {
                Some(ref mut node) => node.insert(key, value),
                None => {
                    self.right = Some(Box::from(Node::new(key, value)));
                    true
                }
            },
            Ordering::Equal => false,
        }
    }

    // pub fn remove(&mut self, key: &K) -> Result<(), RemoveError> {
    //     match key.cmp(self.key()) {
    //         Ordering::Less => todo!(),
    //         Ordering::Greater => todo!(),
    //         Ordering::Equal => Err(RemoveError::RemoveMeIsRoot),
    //     }
    // }

    /// returns a reference to the `Entry` with key `key`
    pub fn entry(&self, key: &K) -> Option<&Entry<K, V>> {
        match key.cmp(self.key()) {
            Ordering::Less => match self.left {
                Some(ref node) => node.entry(key),
                None => None,
            },
            Ordering::Greater => match self.right {
                Some(ref node) => node.entry(key),
                None => None,
            },
            Ordering::Equal => Some(&self.entry),
        }
    }

    /// returns a mutable reference to the `Entry` with key `key`
    pub fn entry_mut(&mut self, key: &K) -> Option<&mut Entry<K, V>> {
        match key.cmp(self.entry.key()) {
            Ordering::Less => match self.left {
                Some(ref mut entry) => entry.entry_mut(key),
                None => None,
            },
            Ordering::Greater => match self.right {
                Some(ref mut node) => node.entry_mut(key),
                None => None,
            },
            Ordering::Equal => Some(&mut self.entry),
        }
    }

    pub fn merge(&mut self, other: Node<K, V>) {
        todo!()
    }

    pub fn contains(&self, key: &K) -> bool {
        match key.cmp(self.entry.key()) {
            Ordering::Less => match self.left {
                Some(ref node) => node.contains(key),
                None => false,
            },
            Ordering::Greater => match self.right {
                Some(ref node) => node.contains(key),
                None => false,
            },
            Ordering::Equal => true,
        }
    }

    pub fn size(&self) -> usize {
        let mut size = 1;
        if let Some(ref node) = self.right {
            size += node.size();
        }
        if let Some(ref entry) = self.left {
            size += entry.size();
        }
        size
    }

    /// returns a reference to the smallest entry
    pub fn smallest(&self) -> &Entry<K, V> {
        match self.left {
            Some(ref entry) => entry.smallest(),
            None => &self.entry,
        }
    }

    /// returns a mutable reference to the smallest entry
    pub fn smallest_mut(&mut self) -> &mut Entry<K, V> {
        match self.left {
            Some(ref mut entry) => entry.smallest_mut(),
            None => &mut self.entry,
        }
    }
    /// returns a reference to the largest entry
    pub fn largest(&self) -> &Entry<K, V> {
        match self.right {
            Some(ref entry) => entry.largest(),
            None => &self.entry,
        }
    }

    /// returns a mutable reference to the largest entry
    pub fn largest_mut(&mut self) -> &mut Entry<K, V> {
        match self.left {
            Some(ref mut entry) => entry.largest_mut(),
            None => &mut self.entry,
        }
    }
}

impl<K, V> RemoveableNode<K> for Option<Box<Node<K, V>>>
where
    K: Ord,
{
    fn remove(&mut self, key: &K) -> bool {
        if let Some(ref mut node) = self {
            match key.cmp(node.key()) {
                Ordering::Less => return node.left.remove(key),
                Ordering::Greater => return node.right.remove(key),
                Ordering::Equal => {
                    if let Some(ref mut smaller_node) = node.left {
                        let parent = smaller_node;
                        loop {
                            match parent.right {
                                Some(ref mut larger_node) => {
                                    if larger_node.right.is_none() {
                                        std::mem::swap(
                                            &mut node.entry,
                                            &mut larger_node.entry,
                                        );
                                        parent.right = larger_node.left.take();
                                        break;
                                    }
                                }
                                None => {
                                    std::mem::swap(
                                        &mut node.entry,
                                        &mut parent.entry,
                                    );
                                    parent.right = None;
                                    break;
                                }
                            }
                        }
                    } else if let Some(ref mut larger_node) = node.right {
                        let parent = larger_node;
                        loop {
                            match parent.left {
                                Some(ref mut smaller_node) => {
                                    if smaller_node.right.is_none() {
                                        std::mem::swap(
                                            &mut node.entry,
                                            &mut smaller_node.entry,
                                        );
                                        parent.left = smaller_node.right.take();
                                        break;
                                    }
                                }
                                None => {
                                    std::mem::swap(
                                        &mut node.entry,
                                        &mut parent.entry,
                                    );
                                    parent.left = None;
                                    break;
                                }
                            }
                        }
                    } else {
                        *self = None;
                    }
                    return true;
                }
            };
        }
        false
    }
}
