//! a singly-linked list that allows for constant time insertion at the front
use crate::linked_list::node::Node;
pub struct LinkedList<T: PartialEq> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T: PartialEq> LinkedList<T> {
    /// appends a new element `key` to `self`
    pub fn push(&mut self, key: T) {
        match self.head {
            Some(ref mut node) => node.push(key),
            None => self.head = Some(Box::from(Node::new(key))),
        };
        self.len += 1;
    }

    /// returns an optional reference to the element at index `index`
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        match self.head {
            Some(ref node) => node.get(index),
            None => None,
        }
    }

    /// returns an optional mutable reference to the element at index `index`
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.head {
            Some(ref mut node) => node.get_mut(index),
            None => None,
        }
    }

    /// inserts `key` at index `index`
    #[inline]
    pub fn insert(&mut self, index: usize, key: T) {
        if index == 0 {
            return self.add(key);
        }
        match self.head {
            Some(ref mut node) => node.insert(index, key),
            None => self.head = Some(Box::from(Node::new(key))),
        }
        self.len += 1;
    }

    /// removes the last element, and returns it
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            Some(ref mut node) => {
                self.len -= 1;
                if self.len == 0 {
                    // let temp = node.key;
                    // self.head = None;
                    // return Some(temp);
                    todo!();
                }
                Some(node.pop())
            }
            None => None,
        }
    }

    /// removes all elements at index `index` and after
    #[inline]
    pub fn trunc(&mut self, index: usize) {
        if let Some(ref mut node) = self.head {
            self.len = index;
            if self.len == 0 {
                return self.head = None;
            }
            node.trunc(index - 1);
        }
    }

    /// sets the element at index `index` to `key`
    /// if no such element exists, nothing happens
    #[inline]
    pub fn set(&mut self, index: usize, key: T) {
        if let Some(ref mut node) = self.head {
            node.set(index, key);
        }
    }

    /// swaps elements at indexes `first_index` and `second_index`
    /// if no such element exists, nothing happens
    #[inline]
    pub fn swap(&mut self, first_index: usize, second_index: usize) {
        if let Some(ref mut node) = self.head {
            node.swap(first_index, second_index);
        }
    }
    /// creates an empty `LinkedList`
    #[inline]
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    /// returns the number of elements in `self`
    #[inline]
    pub fn size(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn add(&mut self, key: T) {
        self.head = Some(Box::from(Node {
            key,
            next: self.head.take(),
        }));
        self.len += 1;
    }

    #[inline]
    pub fn remove(&mut self, key: T) {
        if let Some(ref mut node) = self.head {
            self.len -= 1;
            if self.len == 0 {
                return self.head = node.next.take();
            }
            node.remove(key);
        }
    }

    /// appends `other` to `self`
    #[inline]
    pub fn concat(&mut self, other: LinkedList<T>) {
        if let Some(other_node) = other.head {
            self.len += other.len;
            match self.head {
                Some(ref mut node) => node.concat(*other_node),
                None => self.head = Some(other_node),
            }
        }
    }

    #[inline]
    pub fn contains(&self, key: T) -> bool {
        match self.head {
            Some(ref node) => node.contains(key),
            None => false,
        }
    }
}

impl<T: PartialEq> From<Node<T>> for LinkedList<T> {
    fn from(value: Node<T>) -> Self {
        let len = value.length();
        LinkedList { head: Some(Box::from(value)), len, }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut linked_list = LinkedList::new();
        linked_list.add(5);
        linked_list.add(3);

        assert_eq!(linked_list.size(), 2);
        assert_eq!(linked_list.head.unwrap().key, 3);
    }

    #[test]
    fn get() {
        let mut linked_list = LinkedList::new();
        linked_list.add(5);
        linked_list.add(3);

        assert_eq!(linked_list.get(0), Some(3).as_ref());
        assert_eq!(linked_list.get(1), Some(5).as_ref());
        assert_eq!(linked_list.get(2), None);
    }

    #[test]
    fn push() {
        let mut linked_list = LinkedList::new();
        linked_list.push(5);
        linked_list.push(3);

        assert_eq!(linked_list.get(0), Some(5).as_ref());
        assert_eq!(linked_list.get(1), Some(3).as_ref());
    }

    #[test]
    fn remove() {
        let mut linked_list = LinkedList::new();
        linked_list.add(5);
        linked_list.add(3);

        linked_list.remove(3);
        linked_list.add(4);

        linked_list.remove(5);

        assert_eq!(linked_list.size(), 1);
        assert_eq!(linked_list.get(0), Some(4).as_ref());
    }

    #[test]
    fn contains() {
        let mut linked_list = LinkedList::new();
        linked_list.add(5);
        linked_list.add(3);

        assert!(linked_list.contains(5));
        assert!(linked_list.contains(3));
        assert!(!linked_list.contains(8));
    }
}
