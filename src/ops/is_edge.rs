//! A trait to check if an edge exists between two vertices
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::ops::IsEdge,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([0, 1]),
//! ];
//!
//! assert!(!graph.is_edge(0, 0));
//! assert!(graph.is_edge(0, 1));
//! assert!(graph.is_edge(0, 2));
//! assert!(graph.is_edge(1, 0));
//! assert!(!graph.is_edge(1, 1));
//! assert!(!graph.is_edge(1, 2));
//! assert!(graph.is_edge(2, 0));
//! assert!(graph.is_edge(2, 1));
//! assert!(!graph.is_edge(2, 2));
//! ```

use {
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to check if an edge exists between two vertices
///
/// # How can I implement `IsEdge`?
///
/// Provide an implementation of `is_edge` that returns `true` if there is an
/// edge from `s` to `t`.
///
/// ```
/// use {
///     graaf::ops::IsEdge,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: Vec<HashSet<usize>>,
/// }
///
/// impl IsEdge for Graph {
///     fn is_edge(&self, s: usize, t: usize) -> bool {
///         self.edges.get(s).map_or(false, |set| set.contains(&t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::ops::IsEdge,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([0, 1]),
/// ];
///
/// assert!(!graph.is_edge(0, 0));
/// assert!(graph.is_edge(0, 1));
/// assert!(graph.is_edge(0, 2));
/// assert!(graph.is_edge(1, 0));
/// assert!(!graph.is_edge(1, 1));
/// assert!(!graph.is_edge(1, 2));
/// assert!(graph.is_edge(2, 0));
/// assert!(graph.is_edge(2, 1));
/// assert!(!graph.is_edge(2, 2));
/// ```
///
/// # Properties
///
/// ## `IsEdge` and [`crate::ops::AddEdge`]
///
/// Types that also implement [`crate::ops::AddEdge`] should ensure that the
/// following property holds for every `graph`, `s`, and `t` of the given types:
///
/// ```
/// use graaf::ops::{
///     AddEdge,
///     IsEdge,
/// };
///
/// fn prop_add_edge_is_edge<G, W>(graph: &mut G, s: usize, t: usize) -> bool
/// where
///     G: AddEdge + IsEdge,
/// {
///     graph.add_edge(s, t);
///
///     graph.is_edge(s, t)
/// }
/// ```
///
/// ## `IsEdge` and [`crate::ops::AddWeightedEdge`]
///
/// Types that also implement [`crate::ops::AddWeightedEdge`] should ensure that
/// the following property holds for every `graph`, `s`, `t`, and `w` of the
/// given types:
///
/// ```
/// use graaf::ops::{
///     AddWeightedEdge,
///     IsEdge,
/// };
///
/// fn prop_add_weighted_edge_is_edge<G, W>(graph: &mut G, s: usize, t: usize, w: W) -> bool
/// where
///     G: AddWeightedEdge<W> + IsEdge,
/// {
///     graph.add_weighted_edge(s, t, w);
///
///     graph.is_edge(s, t)
/// }
/// ```
///
/// ## `IsEdge` and [`crate::ops::RemoveEdge`]
///
/// Types that also implement [`crate::ops::RemoveEdge`] should ensure that the
/// following property holds for every `graph`, `s`, and `t` of the given types:
///
/// ```
/// use graaf::ops::{
///     IsEdge,
///     RemoveEdge,
/// };
///
/// fn prop_remove_edge_is_edge<G, W>(graph: &mut G, s: usize, t: usize) -> bool
/// where
///     G: IsEdge + RemoveEdge,
/// {
///     graph.remove_edge(s, t);
///
///     !graph.is_edge(s, t)
/// }
/// ```
pub trait IsEdge {
    /// Checks if there is an edge from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    ///
    /// # Panics
    ///
    /// Implementations may not panic if `s` or `t` are not in the graph.
    fn is_edge(&self, s: usize, t: usize) -> bool;
}

// Vec

impl<H> IsEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<W, H> IsEdge for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

// Arr

impl<const V: usize, H> IsEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<const V: usize, W, H> IsEdge for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

// HashSet

impl<H> IsEdge for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.contains(&(s, t))
    }
}

// HashMap

impl<H> IsEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |set| set.contains(&t))
    }
}

impl<W, H> IsEdge for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |map| map.contains_key(&t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        assert!(!graph.is_edge(0, 0));
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        assert!(!graph.is_edge(0, 0));
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        assert!(!graph.is_edge(0, 0));
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        assert!(!graph.is_edge(0, 0));
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));
    }

    #[test]
    fn hash_set() {
        let graph = HashSet::from([(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]);

        assert!(!graph.is_edge(0, 0));
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([0, 1])),
        ]);

        assert!(!graph.is_edge(0, 0));
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 1)])),
            (1, HashMap::from([(0, 1)])),
            (2, HashMap::from([(0, 1), (1, 1)])),
        ]);

        assert!(!graph.is_edge(0, 0));
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));
    }
}
