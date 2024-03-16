//! a generic binary search tree
//! this tree makes no attempts to maintain balance
//! if you are looking for a self-balancing binary search tree, see AvlTree
use crate::Tree;

/// keys on left branches are less than `self.key`, while values to the right
/// are greater than or equal to `self.key`
pub struct BsTree<T: PartialOrd> {
    key: T,
    left: Option<Box<BsTree<T>>>,
    right: Option<Box<BsTree<T>>>,
}

impl<T: PartialOrd> BsTree<T> {
    pub fn new(key: T) -> BsTree<T> {
        BsTree {
            key,
            left: None,
            right: None,
        }
    }
}

impl<T: PartialOrd> Tree for BsTree<T> {
    type Item = T;

    fn add(&mut self, key: Self::Item) {
        // check left branch
        if self.key < key {
            match &mut self.left {
                Some(ref mut tree) => tree.add(key),
                None => {
                    self.left = Some(Box::new(BsTree {
                        key,
                        left: None,
                        right: None,
                    }));
                }
            }
        } else {
            // right branch
            match &mut self.right {
                Some(ref mut tree) => tree.add(key),
                None => {
                    self.right = Some(Box::new(BsTree {
                        key,
                        left: None,
                        right: None,
                    }));
                }
            }
        }
    }

    fn remove(&mut self, key: Self::Item) {
        let mut remove_me = self.find_mut(key);
        todo!();
    }

    fn find(&self, key: Self::Item) -> Option<&Self> {
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

    fn find_mut(&mut self, key: Self::Item) -> Option<&mut Self> {
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

    fn concat(&mut self, other: Self) {
        todo!()
    }

    fn contains(&self, key: Self::Item) -> bool {
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
}
