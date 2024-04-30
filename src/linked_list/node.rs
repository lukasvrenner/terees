//! a node in `LinkedList`
#[derive(Debug, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>, // scary! public access short term only
}

impl<T> Node<T> {
    /// creates a new `LinkedList`, with `value` as the value and
    /// `None` as `next`
    #[inline]
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }

    /// pushes a new node with value `value` to the end of `self`
    pub fn push(&mut self, value: T) {
        match &mut self.next {
            Some(node) => node.push(value),
            None => self.next = Some(Box::new(Node { value, next: None })),
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
    pub fn insert(&mut self, index: usize, value: T) {
        if let Some(node) = self.at_index_mut(index - 1) {
            let new_node = Node {
                value,
                next: node.next.take(),
            };
            node.next = Some(Box::new(new_node));
        }
    }

    /// removes the last node, and returns its value
    pub fn pop(&mut self) -> T {
        todo!();
    }

    /// truncates off all nodes after `index`
    pub fn trunc(&mut self, index: usize) {
        if let Some(node) = self.at_index_mut(index) {
            node.next = None;
        }
    }

    /// returns an optional reference to the value at index `index`
    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            return Some(&self.value);
        }
        match self.next {
            Some(ref node) => node.get(index - 1),
            None => None,
        }
    }

    /// returns an optional mutable reference to the value at index `index`
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index == 0 {
            return Some(&mut self.value);
        }
        match self.next {
            Some(ref mut node) => node.get_mut(index - 1),
            None => None,
        }
    }

    /// sets the value at index `index` to `value`
    /// if the index is out of bounds, nothing happens
    pub fn set(&mut self, index: usize, value: T) {
        if let Some(node) = self.at_index_mut(index) {
            node.value = value;
        }
    }

    pub fn swap(&mut self, first_index: usize, second_index: usize) {
        todo!();
    }

    /// concatenates `self` and `other`
    pub fn concat(&mut self, other: Node<T>) {
        match &mut self.next {
            Some(node) => node.concat(other),
            None => self.next = Some(Box::new(other)),
        }
    }

    pub fn length(&self) -> usize {
        let mut length = 1;
        if let Some(ref node) = self.next {
            length += node.length();
        }
        length
    }
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn contains(&self, value: T) -> bool {
        if self.value == value {
            return true;
        }
        match &self.next {
            Some(node) => {
                if node.value == value {
                    return true;
                }
                node.contains(value)
            }
            None => false,
        }
    }

    pub fn find(&self, value: T) -> Option<usize> {
        if self.value == value {
            return Some(0);
        }
        if let Some(ref node) = self.next {
            return node.find(value).map(|index| index + 1);
        }
        None
    }

    /// removes the node containing `value`
    /// note: does not consider the first node
    pub fn remove(&mut self, value: T) {
        match &mut self.next {
            Some(node) => {
                if node.value == value {
                    self.next = node.next.take();
                } else {
                    node.remove(value);
                }
            }
            None => (),
        }
    }
}
