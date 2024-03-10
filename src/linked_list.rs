use crate::Tree;

#[derive(Debug)]
pub struct LinkedList<T: PartialEq>(Option<Node<T>>);

#[derive(Debug)]
struct Node<T: PartialEq> {
    key: T,
    next: Box<LinkedList<T>>,
}

impl<T: PartialEq> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList(None)
    }

    pub fn next(&self) -> Option<&LinkedList<T>> {
        Some(self.0.as_ref()?.next.as_ref())
    }

    pub fn next_mut(&mut self) -> Option<&mut LinkedList<T>> {
        Some(self.0.as_mut()?.next.as_mut())
    }
}

impl<T: PartialEq> Tree for LinkedList<T> {
    type Item = T;

    /// adds 'key' to the beginning of 'self'
    /// because it adds to the beginning, it has O(1) time complexity
    fn add(&mut self, key: Self::Item) {
        let current_head = self.0.take();
        self.0 = Some(Node {
            key,
            next: Box::new(LinkedList(current_head)),
        });
    }

    fn find(&self, key: Self::Item) -> Option<&Self> {
        if self
            .0
            .as_ref()
            .is_some_and(|unwrapped| unwrapped.key == key)
        {
            return Some(self);
        }
        if self.0.is_none() {
            return None;
        }
        self.next()?.find(key)
    }

    fn find_mut(&mut self, key: Self::Item) -> Option<&mut Self> {
        if self
            .0
            .as_ref()
            .is_some_and(|unwrapped| unwrapped.key == key)
        {
            return Some(self);
        }
        if self.0.is_none() {
            return None;
        }
        self.next_mut()?.find_mut(key)
    }

    /// removes `self` from the list
    /// does *not* remove sub-elements
    fn drop(&mut self) {
        self.0 = self.0.take().map(|node| node.next.0).flatten();
    }

    /// concatenates `self` and `other`
    fn concat(&mut self, other: Self) {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn add() {
//         let mut linked_list = LinkedList::new();
//         linked_list.add(5);
//         assert_eq!(linked_list, LinkedList(Some(Node{key: 5, next: Box::new(LinkedList(None))})));
//     }
// }
