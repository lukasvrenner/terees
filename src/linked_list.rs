use crate::Tree;

#[derive(Debug, PartialEq)]
pub struct LinkedList<T: PartialEq> {
    key: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: PartialEq> LinkedList<T> {
    /// creates a new `LinkedList`, with `key` as the key and
    /// `None` as `next`
    pub fn new(key: T) -> LinkedList<T> {
        LinkedList { key, next: None }
    }

    /// appends a new node with key `key` to `self`
    pub fn append(&mut self, key: T) {
        match &mut self.next {
            Some(node) => node.append(key),
            None => self.next = Some(Box::new(LinkedList { key, next: None })),
        }
    }

    /// returns an optional reference to the nth node
    pub fn nth(&self, n: usize) -> Option<&Self> {
        if n == 0 {
            return Some(self);
        }
        match &self.next {
            Some(node) => node.nth(n - 1),
            None => None,
        }
    }

    /// returns an optional mutable reference to the nth node
    pub fn nth_mut(&mut self, n: usize) -> Option<&mut Self> {
        if n == 0 {
            return Some(self);
        }
        match &mut self.next {
            Some(node) => node.nth_mut(n - 1),
            None => None,
        }
    }

    /// inserts a new node at index `index`
    pub fn insert(&mut self, index: usize, key: T) {
        todo!()
    }

    /// removes the last node, and returns its key
    pub fn pop(&mut self) -> T {
        todo!();
    }
}

impl<T: PartialEq> Tree for LinkedList<T> {
    type Item = T;

    /// adds `key` to the beginning of `self`
    /// because it adds to the beginning, it has O(1) time complexity
    fn add(&mut self, key: Self::Item) {
        let mut new_head = LinkedList { key, next: None };
        std::mem::swap(self, &mut new_head);
        self.next = Some(Box::new(new_head));
    }

    fn find(&self, key: Self::Item) -> Option<&Self> {
        if self.key == key {
            return Some(self);
        }
        match &self.next {
            Some(node) => node.find(key),
            None => None,
        }
    }

    fn find_mut(&mut self, key: Self::Item) -> Option<&mut Self> {
        if self.key == key {
            return Some(self);
        }
        match &mut self.next {
            Some(node) => node.find_mut(key),
            None => None,
        }
    }

    fn remove(&mut self, key: Self::Item) {
        todo!()
    }

    /// concatenates `self` and `other`
    fn concat(&mut self, other: Self) {
        match &mut self.next {
            Some(node) => node.concat(other),
            None => self.next = Some(Box::new(other)),
        }
    }
}
