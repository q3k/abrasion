use crate::render::renderable::ResourceID;

/// A map from ResourceID to Vec<T>, useful for building up information about
/// a scene. This implementation is currently naive, but is pretty fast
/// compared to a HashMap as it performs no hashing - just uses ResourceIDs.
/// In the future, this could be ported over to a tree implemention from the
/// current naive vector-based implementation.
pub struct ResourceMap<T> {
    pub resources: Vec<(ResourceID, Vec<T>)>,
}

impl<T> ResourceMap<T> {
    pub fn new() -> Self {
        Self {
            resources: vec![],
        }
    }

    /// Add to a Resource's corresponding vector, creating an empty one first
    /// if needed.
    pub fn add(&mut self, r: ResourceID, t: T) {
        for (res, v) in self.resources.iter_mut() {
            if *res == r {
                v.push(t);
                return;
            }
        }

        self.resources.push((r, vec![t]));
    }
}
