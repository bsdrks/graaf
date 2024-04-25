//! A trait to generate linear graphs, also known as path graphs
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

/// A trait to generate linear graphs, also known as path graphs
///
/// # How can I implement `Linear`?
///
/// Provide an implementation of `linear` that generates a linear graph with `v`
/// vertices for your type.
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

        let mut graph = Self::with_capacity(v);

        for i in 0..v - 1 {
            graph.push(vec![i + 1]);
        }

        graph.push(Vec::new());

        graph
    }
}

impl Linear for Vec<BTreeSet<usize>> {
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::with_capacity(0);
        }

        let mut graph = Self::with_capacity(v);

        for i in 0..v - 1 {
            graph.push(BTreeSet::from([i + 1]));
        }

        graph.push(BTreeSet::new());

        graph
    }
}

impl<H> Linear for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
{
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::with_capacity(0);
        }

        let mut graph = Self::with_capacity(v);

        for s in 0..v - 1 {
            let mut out = HashSet::with_hasher(H::default());
            let _ = out.insert(s + 1);

            graph.push(out);
        }

        graph.push(HashSet::with_hasher(H::default()));

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
