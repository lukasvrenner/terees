use crate::Tree;

#[derive(Debug)]
pub struct LinkedList<T: PartialEq>(Option<Node<T>>);

#[derive(Debug)]
struct Node<T: PartialEq> {
    key: T,
    next: Box<LinkedList<T>>
}

impl<T: PartialEq> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList(None)
    }

    pub fn next(&self) -> Option<&LinkedList<T>> {
        Some(self.0.as_ref()?.next.as_ref())
    }
}

impl<T: PartialEq> Tree for LinkedList<T> {
    type Item = T;

    /// adds 'key' to the beginning of 'self'
    /// because it adds to the beginning, it has O(1) time complexity
    fn add(&mut self, key: Self::Item) {
        let current_head = self.0.take();
        self.0 = Some(Node{key, next: Box::new(LinkedList(current_head))});
    }

    fn find(&self, key: Self::Item) -> Option<&Self> {
        if self.0.as_ref().is_some_and(|unwrapped| unwrapped.key == key) {
            return Some(self);
        }
        if self.0.is_none() {
            return None;
        }
        self.0.as_ref()?.next.find(key)
    }

    fn delete(&self, key: Self::Item) {
        todo!()
    }
}
