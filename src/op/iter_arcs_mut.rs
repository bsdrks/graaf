//! A trait that returns an iterator that allows modifying all arcs with a given
//! source vertex in an unweighted digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterArcsMut;
//!
//! let digraph = &mut vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert!(digraph.iter_arcs_mut(0).eq(&mut [1, 2]));
//! assert!(digraph.iter_arcs_mut(1).eq(&mut [0, 2, 3]));
//! assert!(digraph.iter_arcs_mut(2).eq(&mut [0, 1, 3]));
//! assert!(digraph.iter_arcs_mut(3).eq(&mut [1, 2]));
//!
//! for t in digraph.iter_arcs_mut(0) {
//!     *t = (*t + 2) % 4;
//! }
//!
//! assert!(digraph.iter_arcs_mut(0).eq(&mut [3, 0]));
//! assert!(digraph.iter_arcs_mut(1).eq(&mut [0, 2, 3]));
//! assert!(digraph.iter_arcs_mut(2).eq(&mut [0, 1, 3]));
//! assert!(digraph.iter_arcs_mut(3).eq(&mut [1, 2]));
//! ```

extern crate alloc;

use {
    alloc::collections::BTreeMap,
    core::hash::BuildHasher,
    std::collections::HashMap,
};

/// A trait that returns an iterator that allows modifying all arcs with a given
/// source vertex in an unweighted digraph
///
/// # How can I implement `IterArcsMut`?
///
/// Provide an implementation of `iter_arcs_mut` that returns an iterator that
/// allows modifying all arcs with a given source vertex.
///
/// ```
/// use graaf::op::IterArcsMut;
///
/// struct Graph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl IterArcsMut for Graph {
///     fn iter_arcs_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
///         self.arcs[s].iter_mut()
///     }
/// }
///
/// let mut digraph = Graph {
///     arcs: vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]],
/// };
///
/// assert!(digraph.iter_arcs_mut(0).eq(&mut [1, 2]));
/// assert!(digraph.iter_arcs_mut(1).eq(&mut [0, 2, 3]));
/// assert!(digraph.iter_arcs_mut(2).eq(&mut [0, 1, 3]));
/// assert!(digraph.iter_arcs_mut(3).eq(&mut [1, 2]));
///
/// for t in digraph.iter_arcs_mut(0) {
///     *t = (*t + 2) % 4;
/// }
///
/// assert!(digraph.iter_arcs_mut(0).eq(&mut [3, 0]));
/// assert!(digraph.iter_arcs_mut(1).eq(&mut [0, 2, 3]));
/// assert!(digraph.iter_arcs_mut(2).eq(&mut [0, 1, 3]));
/// assert!(digraph.iter_arcs_mut(3).eq(&mut [1, 2]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterArcsMut;
///
/// let digraph = &mut vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert!(digraph.iter_arcs_mut(0).eq(&mut [1, 2]));
/// assert!(digraph.iter_arcs_mut(1).eq(&mut [0, 2, 3]));
/// assert!(digraph.iter_arcs_mut(2).eq(&mut [0, 1, 3]));
/// assert!(digraph.iter_arcs_mut(3).eq(&mut [1, 2]));
///
/// for t in digraph.iter_arcs_mut(0) {
///     *t = (*t + 2) % 4;
/// }
///
/// assert!(digraph.iter_arcs_mut(0).eq(&mut [3, 0]));
/// assert!(digraph.iter_arcs_mut(1).eq(&mut [0, 2, 3]));
/// assert!(digraph.iter_arcs_mut(2).eq(&mut [0, 1, 3]));
/// assert!(digraph.iter_arcs_mut(3).eq(&mut [1, 2]));
/// ```
pub trait IterArcsMut {
    /// Returns an iterator over all arcs with a given source vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn iter_arcs_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize>;
}

impl IterArcsMut for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_arcs_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self[s].iter_mut()
    }
}

impl IterArcsMut for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_arcs_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self[s].iter_mut()
    }
}

impl<const V: usize> IterArcsMut for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_arcs_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self[s].iter_mut()
    }
}

impl IterArcsMut for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_arcs_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self.get_mut(&s).unwrap().iter_mut()
    }
}

impl<H> IterArcsMut for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_arcs_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self.get_mut(&s).unwrap().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_iter_arcs_mut_stable {
        ($digraph:expr) => {
            assert!($digraph.iter_arcs_mut(0).eq(&mut [1, 2]));
            assert!($digraph.iter_arcs_mut(1).eq(&mut [0, 2, 3]));
            assert!($digraph.iter_arcs_mut(2).eq(&mut [0, 1, 3]));
            assert!($digraph.iter_arcs_mut(3).eq(&mut [1, 2]));

            for t in $digraph.iter_arcs_mut(0) {
                *t = (*t + 2) % 4;
            }

            assert!($digraph.iter_arcs_mut(0).eq(&mut [3, 0]));
            assert!($digraph.iter_arcs_mut(1).eq(&mut [0, 2, 3]));
            assert!($digraph.iter_arcs_mut(2).eq(&mut [0, 1, 3]));
            assert!($digraph.iter_arcs_mut(3).eq(&mut [1, 2]));
        };
    }

    #[test]
    fn vec_vec() {
        let digraph = &mut vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_arcs_mut_stable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let digraph: &mut [Vec<usize>] =
            &mut [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_arcs_mut_stable!(digraph);
    }

    #[test]
    fn arr_vec() {
        let digraph = &mut [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_arcs_mut_stable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let digraph = &mut BTreeMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_iter_arcs_mut_stable!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let digraph = &mut HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_iter_arcs_mut_stable!(digraph);
    }
}
