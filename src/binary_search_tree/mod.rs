pub mod map;
mod node;
pub mod set;

trait RemoveableNode<K> {
    fn remove(&mut self, key: &K) -> bool;
}
