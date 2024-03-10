mod linked_list;
pub trait Tree {
    type Item;

    /// adds 'key' to the `self`
    fn add(&mut self, key: Self::Item);

    /// deletes 'key' from 'self'
    fn delete(&self, key: Self::Item);

    /// returns the optional subtree containing `key`
    fn find(&self, key: Self::Item) -> Option<&Self>;
}

