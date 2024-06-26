use super::node::Node;
pub struct BTreeMap<K, V>
where
    K: Ord,
{
    size: usize,
    root: Option<Box<Node<K, V>>>,
}
