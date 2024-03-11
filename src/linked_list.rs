use crate::Tree;

#[derive(Debug)]
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
    pub fn push(&mut self, key: T) {
        match &mut self.next {
            Some(node) => node.push(key),
            None => self.next = Some(Box::new(LinkedList { key, next: None })),
        }
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
        self.next.as_deref()?.find(key)
    }

    fn find_mut(&mut self, key: Self::Item) -> Option<&mut Self> {
        if self.key == key {
            return Some(self);
        }
        self.next.as_deref_mut()?.find_mut(key)
    }

    fn remove(&mut self, key: Self::Item) {
        todo!()
    }

    /// concatenates `self` and `other`
    fn concat(&mut self, other: Self) {
        todo!();
    }
}
