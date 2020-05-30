use std::cmp::PartialEq;

/// A map from ResourceID to Vec<T>, useful for building up information about
/// a scene. This implementation is currently naive, but is pretty fast
/// compared to a HashMap as it performs no hashing - just uses ResourceIDs.
/// In the future, this could be ported over to a tree implemention from the
/// current naive vector-based implementation.
pub struct ResourceMap<K, V> {
    pub resources: Vec<(K, Vec<V>)>,
}

impl<K: PartialEq, V> ResourceMap<K, V> {
    pub fn new() -> Self {
        Self {
            resources: vec![],
        }
    }

    /// Add to a Resource's corresponding vector, creating an empty one first
    /// if needed.
    pub fn add(&mut self, k: K, t: V) {
        for (k2, v) in self.resources.iter_mut() {
            if *k2 == k {
                v.push(t);
                return;
            }
        }

        self.resources.push((k, vec![t]));
    }
}
