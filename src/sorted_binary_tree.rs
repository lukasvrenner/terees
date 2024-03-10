use crate::Tree;

/// keys on left branches are less than `self.0.key`, while values to the right
/// are greater than or equal to `self.0.key`
pub struct SortedBinaryTree<T: PartialOrd>(Option<Node<T>>);

struct Node<T: PartialOrd> {
    key: T,
    left: Box<SortedBinaryTree<T>>,
    right: Box<SortedBinaryTree<T>>,
}

impl<T: PartialOrd> SortedBinaryTree<T> {
    /// returns an optional immutable reference to the left branch of the tree
    pub fn left(&self) -> Option<&SortedBinaryTree<T>> {
        Some(self.0.as_ref()?.left.as_ref())
    }

    /// returns an optional mutable reference to the left branch of the tree
    pub fn left_mut(&mut self) -> Option<&mut SortedBinaryTree<T>> {
        Some(self.0.as_mut()?.left.as_mut())
    }

    /// returns an optional immutable reference to the right branch of the tree
    pub fn right(&self) -> Option<&SortedBinaryTree<T>> {
        Some(self.0.as_ref()?.right.as_ref())
    }

    /// returns an optional mutable reference to the right branch of the tree
    pub fn right_mut(&mut self) -> Option<&mut SortedBinaryTree<T>> {
        Some(self.0.as_mut()?.right.as_mut())
    }
}

impl<T: PartialOrd> SortedBinaryTree<T> {
    fn new() -> SortedBinaryTree<T> {
        SortedBinaryTree(None)
    }
}

impl<T: PartialOrd> Tree for SortedBinaryTree<T> {
    type Item = T;
    
    /// adds a new node with a key `key` to the tree
    fn add(&mut self, key: Self::Item) {
        // check left branch
        if self.0.as_mut().is_some_and(|node| key < node.key) {
            match self.left_mut() {
                Some(tree) => return tree.add(key),
                None => {
                    self.0.as_mut().unwrap().left =
                        Box::new(SortedBinaryTree(Some(Node {
                            key,
                            left: Box::new(SortedBinaryTree(None)),
                            right: Box::new(SortedBinaryTree(None)),
                        })));
                    return;
                }
            }
        } 
        // check right branch
        if self.0.as_mut().is_some_and(|node| key >= node.key) {
            match self.right_mut() {
                Some(tree) => return tree.add(key),
                None => {
                    self.0.as_mut().unwrap().right =
                        Box::new(SortedBinaryTree(Some(Node {
                            key,
                            left: Box::new(SortedBinaryTree(None)),
                            right: Box::new(SortedBinaryTree(None)),
                        })));
                    return;
                }
            }
        }
        *self = SortedBinaryTree(Some(Node {
            key,
            left: Box::new(SortedBinaryTree(None)),
            right: Box::new(SortedBinaryTree(None)),
        }))
    }

    fn remove(&mut self, key: Self::Item) {
        todo!();
    }

    fn find(&self, key: Self::Item) -> Option<&Self> {
        if self.0.as_ref().is_some_and(|node| key < node.key) {
            return self.left()?.find(key);
        }
        if self.0.as_ref().is_some_and(|node| key > node.key) {
            return self.right()?.find(key);
        }
        if self.0.is_none() {
            return None;
        }
        Some(self)
    }

    fn find_mut(&mut self, key: Self::Item) -> Option<&mut Self> {
        if self.0.as_mut().is_some_and(|node| key < node.key) {
            return self.left_mut()?.find_mut(key);
        }
        if self.0.as_mut().is_some_and(|node| key > node.key) {
            return self.right_mut()?.find_mut(key);
        }
        if self.0.is_none() {
            return None;
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
