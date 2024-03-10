pub trait Tree {
    type Item;

    /// adds 'key' to the `self`
    fn add(&mut self, key: Self::Item);

    /// deletes 'key' from 'self'
    fn delete(&self, key: Self::Item);

    /// returns the optional subtree containing `key`
    fn find(&self, key: Self::Item) -> Option<&Self>;
}

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>)
where
    T: PartialEq;

impl<T: PartialEq> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList(None)
    }
}

impl<T: PartialEq> Tree for LinkedList<T> {
    type Item = T;

    /// adds 'key' to the beginning of 'self'
    /// because it adds to the beginning, it has O(1) time complexity
    fn add(&mut self, key: Self::Item) {
        let current_head = self.0.take();
        self.0 = Some((key, Box::new(LinkedList(current_head))));
    }

    fn find(&self, key: Self::Item) -> Option<&Self> {
        if self.0.as_ref().is_some_and(|unwrapped| unwrapped.0 == key) {
            return Some(self);
        }
        self.0.as_ref()?.1.find(key)
    }

    fn delete(&self, key: Self::Item) {
        todo!()
    }
}
