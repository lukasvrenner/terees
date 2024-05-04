use crate::entry::Entry;
const CAPACITY: usize = 6;
const MIN: usize = CAPACITY / 2;
const MAX_CHILDREN: usize = CAPACITY + 1;

pub struct Node<K, V> where K: Ord {
    len: usize,
    entries: [Option<Entry<K, V>>; CAPACITY],
    children: [Option<Box<Node<K, V>>>; MAX_CHILDREN],
}

impl<K, V> Node<K, V> where K: Ord {

}
