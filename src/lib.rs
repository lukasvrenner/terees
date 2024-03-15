pub mod binary_search_tree;
pub mod linked_list;

pub trait Tree {
    type Item;

    /// adds `key` to the `self`
    fn add(&mut self, key: Self::Item);

    /// removes the node containing `key` from `self`
    fn remove(&mut self, key: Self::Item);

    /// returns an optional reference to the node containing `key`
    fn find(&self, key: Self::Item) -> Option<&Self>;

    /// returns an optional mutable reference to the node containing `key`
    fn find_mut(&mut self, key: Self::Item) -> Option<&mut Self>;

    /// concatenates trees
    /// does not necessarily maintain original order
    fn concat(&mut self, other: Self);
}
