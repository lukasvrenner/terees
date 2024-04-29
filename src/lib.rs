pub mod binary_search_tree;
pub mod linked_list;

pub trait Tree {
    type Item;

    /// creates an empty `Self`
    fn new() -> Self;

    /// adds `key` to `self`
    fn add(&mut self, key: Self::Item);

    /// removes the node containing `key` from `self`
    fn remove(&mut self, key: Self::Item);

    /// concatenates `other` to `self`
    /// note: does not necessarily maintain original order
    fn concat(&mut self, other: Self);

    /// returns true if `self` contains a node with key `key`.
    /// otherwise, returns false
    fn contains(&self, key: Self::Item) -> bool;

    /// returns the number of nodes in `self`
    fn size(&self) -> usize;
}
