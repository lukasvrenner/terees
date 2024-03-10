mod linked_list;
mod sorted_binary_tree;

pub trait Tree {
    type Item;

    /// adds 'key' to the `self`
    fn add(&mut self, key: Self::Item);

    /// removes the node containing 'key' from 'self'
    fn remove(&mut self, key: Self::Item) {
        self.find_mut(key).map(|node| node.drop());
    }

    /// returns an optional immutable reference to thesubtree containing `key`
    fn find(&self, key: Self::Item) -> Option<&Self>;

    /// returns an optional mutable reference to the subtree containing 'key'
    fn find_mut(&mut self, key: Self::Item) -> Option<&mut Self>;

    /// removes 'self' from tree
    fn drop(&mut self);

    /// concatenates trees
    /// might not ensure the order is the same
    fn concat(&mut self, other: Self);
}
