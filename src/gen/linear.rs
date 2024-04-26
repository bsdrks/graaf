//! A trait to generate linear graphs
//!
//! Linear graphs are also known as path graphs.
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Linear;
//!
//! //
//! assert_eq!(Vec::<Vec<usize>>::linear(0), Vec::<Vec<usize>>::new());
//!
//! // 0
//! assert_eq!(Vec::<Vec<usize>>::linear(1), vec![Vec::new()]);
//!
//! // 0 → 1
//! assert_eq!(Vec::<Vec<usize>>::linear(2), vec![vec![1], Vec::new()]);
//!
//! // 0 → 1 → 2
//! assert_eq!(
//!     Vec::<Vec<usize>>::linear(3),
//!     vec![vec![1], vec![2], Vec::new()]
//! );
//! ```
use {
    super::Empty,
    crate::op::AddEdge,
};

/// A trait to generate linear graphs
///
/// # How can I implement `Linear`?
///
/// Provide an implementation of `linear` that generates a linear graph with `v`
/// vertices.
///
/// ```
/// use {
///     graaf::gen::Linear,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Linear for Graph {
///     fn linear(v: usize) -> Self {
///         Graph {
///             edges: (0..v - 1).map(|s| (s, s + 1)).collect(),
///         }
///     }
/// }
///
/// let graph = Graph::linear(3);
///
/// assert_eq!(graph.edges, HashSet::from([(0, 1), (1, 2)]));
/// ```
pub trait Linear {
    /// Generate a linear graph, also known as a path graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn linear(v: usize) -> Self;
}

impl<G> Linear for G
where
    G: AddEdge + Empty,
{
    fn linear(v: usize) -> Self {
        let mut graph = G::empty(v);

        for s in 0..v {
            let t = s + 1;

            if t < v {
                graph.add_edge(s, t);
            }
        }

        graph
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
            //
            Vec::new(),
            // 0
            vec![Vec::new()],
            // 0 → 1
            vec![vec![1], Vec::new()],
            // 0 → 1 → 2
            vec![vec![1], vec![2], Vec::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::linear(v), g);
        }
    }

    #[test]
    fn vec_btree_set() {
        for (v, g) in [
            //
            Vec::new(),
            // 0
            vec![BTreeSet::new()],
            // 0 → 1
            vec![BTreeSet::from([1]), BTreeSet::new()],
            // 0 → 1 → 2
            vec![BTreeSet::from([1]), BTreeSet::from([2]), BTreeSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<usize>>::linear(v), g);
        }
    }

    #[test]
    fn vec_hash_set() {
        for (v, g) in [
            //
            Vec::new(),
            // 0
            vec![HashSet::new()],
            // 0 → 1
            vec![HashSet::from([1]), HashSet::new()],
            // 0 → 1 → 2
            vec![HashSet::from([1]), HashSet::from([2]), HashSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<usize>>::linear(v), g);
        }
    }
}
