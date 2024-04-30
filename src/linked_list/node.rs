//! a node in `LinkedList`
#[derive(Debug, PartialEq)]
pub struct Node<T: PartialEq> {
    pub key: T,
    pub next: Option<Box<Node<T>>>, // scary! public access short term only
}

impl<T: PartialEq> Node<T> {
    /// creates a new `LinkedList`, with `key` as the key and
    /// `None` as `next`
    #[inline]
    pub fn new(key: T) -> Node<T> {
        Node { key, next: None }
    }

    /// pushes a new node with key `key` to the end of `self`
    pub fn push(&mut self, key: T) {
        match &mut self.next {
            Some(node) => node.push(key),
            None => self.next = Some(Box::new(Node { key, next: None })),
        }
    }

    /// returns an optional mutable reference to node at index `index`
    pub fn at_index_mut(&mut self, n: usize) -> Option<&mut Node<T>> {
        if n == 0 {
            return Some(self);
        }
        match &mut self.next {
            Some(node) => node.at_index_mut(n - 1),
            None => None,
        }
    }

    /// inserts a new node at index `index`
    pub fn insert(&mut self, index: usize, key: T) {
        if let Some(node) = self.at_index_mut(index - 1) {
            let new_node = Node {
                key,
                next: node.next.take(),
            };
            node.next = Some(Box::new(new_node));
        }
    }

    /// removes the last node, and returns its key
    pub fn pop(&mut self) -> T {
        todo!();
    }

    /// truncates off all nodes after `index`
    pub fn trunc(&mut self, index: usize) {
        if let Some(node) = self.at_index_mut(index) {
            node.next = None;
        }
    }

    /// returns an optional reference to the key at index `index`
    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            return Some(&self.key);
        }
        match self.next {
            Some(ref node) => node.get(index - 1),
            None => None,
        }
    }

    /// returns an optional mutable reference to the key at index `index`
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index == 0 {
            return Some(&mut self.key);
        }
        match self.next {
            Some(ref mut node) => node.get_mut(index - 1),
            None => None,
        }
    }

    /// sets the key at index `index` to `key`
    /// if the index is out of bounds, nothing happens
    pub fn set(&mut self, index: usize, key: T) {
        if let Some(node) = self.at_index_mut(index) {
            node.key = key;
        }
    }

    pub fn swap(&mut self, first_index: usize, second_index: usize) {
        todo!();
    }

    /// removes the node containing `key`
    /// note: does not consider the first node
    pub fn remove(&mut self, key: T) {
        match &mut self.next {
            Some(node) => {
                if node.key == key {
                    self.next = node.next.take();
                } else {
                    node.remove(key);
                }
            }
            None => (),
        }
    }

    /// concatenates `self` and `other`
    pub fn concat(&mut self, other: Node<T>) {
        match &mut self.next {
            Some(node) => node.concat(other),
            None => self.next = Some(Box::new(other)),
        }
    }

    pub fn contains(&self, key: T) -> bool {
        if self.key == key {
            return true;
        }
        match &self.next {
            Some(node) => {
                if node.key == key {
                    return true;
                }
                node.contains(key)
            }
            None => false,
        }
    }

    pub fn length(&self) -> usize {
        let mut length = 1;
        if let Some(ref node) = self.next {
            length += node.length();
        }
        length
    }

    pub fn find(&self, key: T) -> Option<usize> {
        if self.key == key {
            return Some(0);
        }
        if let Some(ref node) = self.next {
            return node.find(key).map(|index| index + 1);
        }
        None
    }
}
