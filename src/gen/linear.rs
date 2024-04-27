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

extern crate alloc;

use {
    alloc::collections::BTreeSet,
    core::hash::BuildHasher,
    std::collections::HashSet,
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

impl Linear for Vec<Vec<usize>> {
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![Vec::new(); v];

        for (s, vec) in graph.iter_mut().enumerate().take(v - 1) {
            vec.push(s + 1);
        }

        graph
    }
}

impl Linear for Vec<BTreeSet<usize>> {
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![BTreeSet::new(); v];

        for (s, set) in graph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        graph
    }
}

impl<H> Linear for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![HashSet::with_hasher(H::default()); v];

        for (s, set) in graph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
