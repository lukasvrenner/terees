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

    /// returns an optional reference to the node at index `index`
    pub fn at_index(&self, n: usize) -> Option<&Self> {
        if n == 0 {
            return Some(self);
        }
        match &self.next {
            Some(node) => node.at_index(n - 1),
            None => None,
        }
    }

    /// returns an optional mutable reference to node at index `index`
    pub fn at_index_mut(&mut self, n: usize) -> Option<&mut Self> {
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
        // there is no node before the first node
        if index == 0 {
            return self.add(key);
        }
        if let Some(node) = self.at_index_mut(index - 1) {
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
        if let Some(node) = self.at_index_mut(index) {
            node.next = None;
        }
    }

    /// returns the number of nodes in `self`
    pub fn length(&self) -> usize {
        // start at one because we cannot have an empty list (as of now)
        let mut length = 1;
        let mut next = &self.next;

        while let Some(node) = next {
            next = &node.next;
            length += 1;
        }
        length
    }

    /// returns an optional reference to the key at index `index`
    pub fn get(&self, index: usize) -> Option<&T> {
        todo!();
    }

    /// returns an optional mutable reference to the key at index `index`
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        todo!();
    }

    /// sets the key at index `index` to `key`
    /// if the index is out of bounds, nothing happens
    pub fn set(&mut self, index: usize, key: T) {
        if let Some(node) = self.at_index_mut(index) {
            node.key = key;
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

    fn contains(&self, key: Self::Item) -> bool {
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
}

// TODO:
// add tests for each function
// properly handle empty lists -- so far, lists cannot be empty

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

    #[test]
    fn length() {
        let mut linked_list = LinkedList::new(5);
        linked_list.add(4);
        linked_list.add(3);
        assert_eq!(linked_list.length(), 3);
    }

    #[test]
    fn contains() {
        let mut linked_list = LinkedList::new("hello");
        linked_list.add("world");

        assert!(!linked_list.contains("goodbye"));
        assert!(linked_list.contains("world"));
    }

    #[test]
    fn concat() {
        let mut first_list = LinkedList::new(3);
        first_list.add(5);
        let mut second_list = LinkedList::new(4);
        second_list.add(8);
        first_list.concat(second_list);

        let mut third_list = LinkedList::new(4);
        third_list.add(8);
        third_list.add(3);
        third_list.add(5);

        assert_eq!(first_list, third_list);
    }
}
