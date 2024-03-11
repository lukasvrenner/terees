use crate::Tree;

/// keys on left branches are less than `self.key`, while values to the right
/// are greater than or equal to `self.key`
pub struct SortedBinaryTree<T: PartialOrd> {
    key: T,
    left: Option<Box<SortedBinaryTree<T>>>,
    right: Option<Box<SortedBinaryTree<T>>>,
}

impl<T: PartialOrd> SortedBinaryTree<T> {
    pub fn new(key: T) -> SortedBinaryTree<T> {
        SortedBinaryTree {
            key,
            left: None,
            right: None,
        }
    }
}

impl<T: PartialOrd> Tree for SortedBinaryTree<T> {
    type Item = T;

    fn add(&mut self, key: Self::Item) {
        // check left branch
        if self.key < key {
            match &mut self.left {
                Some(ref mut tree) => return tree.add(key),
                None => {
                    self.left = Some(Box::new(SortedBinaryTree {
                        key,
                        left: None,
                        right: None,
                    }));
                    return;
                }
            }
        }
        // check right branch
        if self.key >= key {
            match &mut self.right {
                Some(ref mut tree) => return tree.add(key),
                None => {
                    self.right = Some(Box::new(SortedBinaryTree {
                        key,
                        left: None,
                        right: None,
                    }));
                    return;
                }
            }
        }
        *self = SortedBinaryTree {
            key,
            left: None,
            right: None,
        };
    }

    fn remove(&mut self, key: Self::Item) {
        todo!();
    }

    fn find(&self, key: Self::Item) -> Option<&Self> {
        if self.key < key {
            return self.left.as_deref()?.find(key);
        }
        if self.key > key {
            return self.right.as_deref()?.find(key);
        }
        Some(self)
    }

    fn find_mut(&mut self, key: Self::Item) -> Option<&mut Self> {
        if self.key < key {
            return self.left.as_deref_mut()?.find_mut(key);
        }
        if self.key > key {
            return self.right.as_deref_mut()?.find_mut(key);
        }
        Some(self)
    }

    fn drop(&mut self) {
        todo!();
    }

    fn concat(&mut self, other: Self) {
        todo!()
    }
}
