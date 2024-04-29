//! a generic binary search tree
//! this tree makes no attempts to maintain balance
//! if you are looking for a self-balancing binary search tree, see AvlTree
use crate::Tree;

pub struct BsTree<T: PartialOrd> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: PartialOrd> Tree for BsTree<T> {
    type Item = T;

    /// creates and empty BsTree<T>
    #[inline]
    fn new() -> Self {
        BsTree { head: None, size: 0 }
    }

    #[inline]
    fn size(&self) -> usize {
        self.size
    }

    #[inline]
    fn contains(&self, key: Self::Item) -> bool {
        match self.head {
            Some(ref node) => node.contains(key),
            None => false,
        }
    }

    fn concat(&mut self, other: Self) {
        self.size += other.size;
        todo!();
    }

    fn remove(&mut self, key: Self::Item) {
        if let Some(ref mut node) = self.head {
            self.size -= 1;
            todo!();
        }
    }

    #[inline]
    fn add(&mut self, key: Self::Item) {
        match self.head {
            Some(ref mut node) => node.add(key),
            None => self.head = Some(Box::from(Node::new(key))),
        }
        self.size += 1;
    }

}

impl<T: PartialOrd> From<Node<T>> for BsTree<T> {
    fn from(value: Node<T>) -> Self {
        let size = value.size();
        BsTree { head: Some(Box::from(value)), size, }
    }
}
/// `left` represents nodes that have smaller `key`s than `self.key`
/// `right` represents nodes that have `key`s greater than or equal to `self.key`
struct Node<T: PartialOrd> {
    key: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
    fn new(key: T) -> Node<T> {
        Node {
            key,
            left: None,
            right: None,
        }
    }
    fn add(&mut self, key: T) {
        // check left branch
        if self.key < key {
            match &mut self.left {
                Some(ref mut tree) => tree.add(key),
                None => {
                    self.left = Some(Box::new(Node::new(key)));
                }
            }
        } else {
            // right branch
            match &mut self.right {
                Some(ref mut tree) => tree.add(key),
                None => {
                    self.right = Some(Box::new(Node::new(key)));
                }
            }
        }
    }

    fn remove(&mut self, key: T) {
        todo!();
    }

    fn find(&self, key: T) -> Option<&Node<T>> {
        if self.key < key {
            match &self.left {
                Some(node) => return node.find(key),
                None => return None,
            }
        }
        if self.key > key {
            match &self.right {
                Some(node) => return node.find(key),
                None => return None,
            }
        }
        Some(self)
    }

    fn find_mut(&mut self, key: T) -> Option<&mut Node<T>> {
        if self.key < key {
            match &mut self.left {
                Some(node) => return node.find_mut(key),
                None => return None,
            }
        }
        if self.key > key {
            match &mut self.right {
                Some(node) => return node.find_mut(key),
                None => return None,
            }
        }
        Some(self)
    }

    fn concat(&mut self, other: Node<T>) {
        todo!()
    }

    fn contains(&self, key: T) -> bool {
        if self.key < key {
            match &self.left {
                Some(node) => return node.contains(key),
                None => return false,
            };
        }
        if self.key > key {
            match &self.right {
                Some(node) => return node.contains(key),
                None => return false,
            };
        }
        true
    }

    fn size(&self) -> usize {
        let mut size = 1;
        if let Some(ref node) = self.right {
            size += node.size();
        }
        if let Some(ref node) = self.left {
            size += node.size();
        }
        size
    }
}
