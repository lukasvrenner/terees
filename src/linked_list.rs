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

    /// pushes a new node with key `key` to the end of `self`
    pub fn push(&mut self, key: T) {
        match &mut self.next {
            Some(node) => node.push(key),
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
        // there is no node before the first node
        if index == 0 {
            return self.add(key);
        }
        if let Some(node) = self.nth_mut(index - 1) {
            let new_node = LinkedList {
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
        if let Some(node) = self.nth_mut(index) {
            node.next = None;
        }
    }

    /// returns the number of nodes in `self`
    pub fn length(&self) -> usize {
        let mut length = 0;
        todo!();
    }

    /// returns true if `self` contains a node that contains `key`
    pub fn contains(&self, key: usize) -> bool {
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

    // TODO: properly handle if the node containing `key` is the first node
    fn remove(&mut self, key: Self::Item) {
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
    fn concat(&mut self, other: Self) {
        match &mut self.next {
            Some(node) => node.concat(other),
            None => self.next = Some(Box::new(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut linked_list = LinkedList::new(5);

        linked_list.add(4);
        assert_eq!(linked_list.key, 4);

        linked_list.next.map(|node| {
            assert_eq!(node.key, 5);
        });
    }

    #[test]
    fn push() {
        let mut linked_list = LinkedList::new(5);
        linked_list.push(4);

        assert_eq!(linked_list.key, 5);

        linked_list.next.map(|node| {
            assert_eq!(node.key, 4);
        });
    }

    #[test]
    fn remove() {
        let mut linked_list = LinkedList::new(5);
        linked_list.add(4);
        linked_list.add(3);
        linked_list.remove(4);

        assert_eq!(linked_list.key, 3);

        linked_list.next.map(|node| {
            assert_eq!(node.key, 5);
        });
    }
}
