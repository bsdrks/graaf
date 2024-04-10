//! A trait to get the weight of a given edge
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::EdgeWeight,
//!     std::collections::HashMap,
//! };
//!
//! let graph = vec![
//!     HashMap::from([(1, 2), (2, 3)]),
//!     HashMap::from([(0, 4)]),
//!     HashMap::from([(0, 7), (1, 8)]),
//! ];
//!
//! assert_eq!(graph.edge_weight(0, 0), None);
//! assert_eq!(graph.edge_weight(0, 1), Some(&2));
//! assert_eq!(graph.edge_weight(0, 2), Some(&3));
//! assert_eq!(graph.edge_weight(1, 0), Some(&4));
//! assert_eq!(graph.edge_weight(1, 1), None);
//! assert_eq!(graph.edge_weight(2, 0), Some(&7));
//! assert_eq!(graph.edge_weight(2, 1), Some(&8));
//! assert_eq!(graph.edge_weight(2, 2), None);
//! ```

use {
    core::hash::BuildHasher,
    std::collections::HashMap,
};

/// A trait to get the weight of a given edge
///
/// # How can I implement `EdgeWeight`?
///
/// Provide an implementation of `edge_weight` that returns the weight of the
/// edge from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::EdgeWeight,
///     std::collections::HashMap,
/// };
///
/// struct Graph {
///     edges: Vec<HashMap<usize, usize>>,
/// }
///
/// impl EdgeWeight<usize> for Graph {
///     fn edge_weight(&self, s: usize, t: usize) -> Option<&usize> {
///         self.edges.get(s).and_then(|m| m.get(&t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::EdgeWeight,
///     std::collections::HashMap,
/// };
///
/// let graph = vec![
///     HashMap::from([(1, 2), (2, 3)]),
///     HashMap::from([(0, 4)]),
///     HashMap::from([(0, 7), (1, 8)]),
/// ];
///
/// assert_eq!(graph.edge_weight(0, 0), None);
/// assert_eq!(graph.edge_weight(0, 1), Some(&2));
/// assert_eq!(graph.edge_weight(0, 2), Some(&3));
/// assert_eq!(graph.edge_weight(1, 0), Some(&4));
/// assert_eq!(graph.edge_weight(1, 1), None);
/// assert_eq!(graph.edge_weight(2, 0), Some(&7));
/// assert_eq!(graph.edge_weight(2, 1), Some(&8));
/// assert_eq!(graph.edge_weight(2, 2), None);
/// ```
pub trait EdgeWeight<W> {
    /// Get the weight of the edge from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    fn edge_weight(&self, s: usize, t: usize) -> Option<&W>;
}

impl<W, H> EdgeWeight<W> for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn edge_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(s).and_then(|m| m.get(&t))
    }
}

impl<W, H> EdgeWeight<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn edge_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(&s).and_then(|m| m.get(&t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec() {
        #[allow(clippy::useless_vec)]
        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.edge_weight(0, 1), Some(&2));
        assert_eq!(graph.edge_weight(0, 2), Some(&3));
        assert_eq!(graph.edge_weight(1, 0), Some(&4));
        assert_eq!(graph.edge_weight(2, 0), Some(&7));
        assert_eq!(graph.edge_weight(2, 1), Some(&8));
    }

    #[test]
    fn slice() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.edge_weight(0, 1), Some(&2));
        assert_eq!(graph.edge_weight(0, 2), Some(&3));
        assert_eq!(graph.edge_weight(1, 0), Some(&4));
        assert_eq!(graph.edge_weight(2, 0), Some(&7));
        assert_eq!(graph.edge_weight(2, 1), Some(&8));
    }

    #[test]
    fn arr() {
        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.edge_weight(0, 1), Some(&2));
        assert_eq!(graph.edge_weight(0, 2), Some(&3));
        assert_eq!(graph.edge_weight(1, 0), Some(&4));
        assert_eq!(graph.edge_weight(2, 0), Some(&7));
        assert_eq!(graph.edge_weight(2, 1), Some(&8));
    }

    #[test]
    fn hash_map() {
        let mut graph = HashMap::new();
        let _ = graph.insert(0, HashMap::from([(1, 2), (2, 3)]));
        let _ = graph.insert(1, HashMap::from([(0, 4)]));
        let _ = graph.insert(2, HashMap::from([(0, 7), (1, 8)]));

        assert_eq!(graph.edge_weight(0, 1), Some(&2));
        assert_eq!(graph.edge_weight(0, 2), Some(&3));
        assert_eq!(graph.edge_weight(1, 0), Some(&4));
        assert_eq!(graph.edge_weight(2, 0), Some(&7));
        assert_eq!(graph.edge_weight(2, 1), Some(&8));
    }
}
