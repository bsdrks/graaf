//! A trait to generate empty graphs
//!
//! Not to be confused with an order-zero graph, which is a graph with no
//! vertices. An empty graph has no edges.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use graaf::gen::Empty;
//!
//! assert_eq!(Vec::<Vec<usize>>::empty(0), Vec::<Vec<usize>>::new());
//! assert_eq!(Vec::<Vec<usize>>::empty(1), vec![Vec::new()]);
//! assert_eq!(Vec::<Vec<usize>>::empty(2), vec![Vec::new(), Vec::new()]);
//! ```

/// A trait to generate empty graphs
///
/// # How can I implement `Empty`?
///
/// Provide an implementation of `empty` that generates an empty graph with `v`
/// vertices. If your type also implements [`AddEdge`](crate::op::AddEdge), then
/// `Empty::empty(v).add_edge(s, t)` may not panic for any `v`, `s < v`, and
/// `t < v`.
///
/// ```
/// use graaf::gen::Empty;
///
/// struct Graph {
///     edges: Vec<Vec<usize>>,
/// }
///
/// impl Empty for Graph {
///     fn empty(v: usize) -> Self {
///         Graph {
///             edges: vec![Vec::new(); v],
///         }
///     }
/// }
///
/// let graph = Graph::empty(3);
///
/// assert_eq!(graph.edges, vec![Vec::new(), Vec::new(), Vec::new()]);
/// ```
pub trait Empty {
    /// Generate an empty graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn empty(v: usize) -> Self;
}

impl<T> Empty for Vec<T>
where
    T: Clone + Default + IntoIterator<Item = usize>,
{
    fn empty(v: usize) -> Self {
        vec![T::default(); v]
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        alloc::collections::BTreeSet,
        std::collections::HashSet,
    };

    #[test]
    fn vec_vec() {
        for (v, g) in [
            Vec::new(),
            vec![Vec::new()],
            vec![Vec::new(), Vec::new()],
            vec![Vec::new(), Vec::new(), Vec::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::empty(v), g);
        }
    }

    #[test]
    fn vec_btree_set() {
        for (v, g) in [
            Vec::new(),
            vec![BTreeSet::new()],
            vec![BTreeSet::new(), BTreeSet::new()],
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<usize>>::empty(v), g);
        }
    }

    #[test]
    fn vec_hash_set() {
        for (v, g) in [
            Vec::new(),
            vec![HashSet::new()],
            vec![HashSet::new(), HashSet::new()],
            vec![HashSet::new(), HashSet::new(), HashSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<usize>>::empty(v), g);
        }
    }
}
