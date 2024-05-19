//! A trait that returns an iterator to mutate the out-neighbors of a vertex in
//! a digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterOutNeighborsMut;
//!
//! let digraph = &mut vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert!(digraph.iter_out_neighbors_mut(0).eq(&mut [1, 2]));
//! assert!(digraph.iter_out_neighbors_mut(1).eq(&mut [0, 2, 3]));
//! assert!(digraph.iter_out_neighbors_mut(2).eq(&mut [0, 1, 3]));
//! assert!(digraph.iter_out_neighbors_mut(3).eq(&mut [1, 2]));
//!
//! for t in digraph.iter_out_neighbors_mut(0) {
//!     *t = (*t + 2) % 4;
//! }
//!
//! assert!(digraph.iter_out_neighbors_mut(0).eq(&mut [3, 0]));
//! assert!(digraph.iter_out_neighbors_mut(1).eq(&mut [0, 2, 3]));
//! assert!(digraph.iter_out_neighbors_mut(2).eq(&mut [0, 1, 3]));
//! assert!(digraph.iter_out_neighbors_mut(3).eq(&mut [1, 2]));
//! ```

extern crate alloc;

use {
    alloc::collections::BTreeMap,
    core::hash::BuildHasher,
    std::collections::HashMap,
};

/// A trait that returns an iterator to mutate the out-neighbors of a vertex in
/// a digraph
///
/// # How can I implement `IterOutNeighborsMut`?
///
/// Provide an implementation of `iter_out_neighbors_mut` that returns an
/// iterator to mutate the out-neighbors of a vertex.
///
/// ```
/// use graaf::op::IterOutNeighborsMut;
///
/// struct Graph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl IterOutNeighborsMut for Graph {
///     fn iter_out_neighbors_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
///         self.arcs[s].iter_mut()
///     }
/// }
///
/// let mut digraph = Graph {
///     arcs: vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]],
/// };
///
/// assert!(digraph.iter_out_neighbors_mut(0).eq(&mut [1, 2]));
/// assert!(digraph.iter_out_neighbors_mut(1).eq(&mut [0, 2, 3]));
/// assert!(digraph.iter_out_neighbors_mut(2).eq(&mut [0, 1, 3]));
/// assert!(digraph.iter_out_neighbors_mut(3).eq(&mut [1, 2]));
///
/// for t in digraph.iter_out_neighbors_mut(0) {
///     *t = (*t + 2) % 4;
/// }
///
/// assert!(digraph.iter_out_neighbors_mut(0).eq(&mut [3, 0]));
/// assert!(digraph.iter_out_neighbors_mut(1).eq(&mut [0, 2, 3]));
/// assert!(digraph.iter_out_neighbors_mut(2).eq(&mut [0, 1, 3]));
/// assert!(digraph.iter_out_neighbors_mut(3).eq(&mut [1, 2]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterOutNeighborsMut;
///
/// let digraph = &mut vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert!(digraph.iter_out_neighbors_mut(0).eq(&mut [1, 2]));
/// assert!(digraph.iter_out_neighbors_mut(1).eq(&mut [0, 2, 3]));
/// assert!(digraph.iter_out_neighbors_mut(2).eq(&mut [0, 1, 3]));
/// assert!(digraph.iter_out_neighbors_mut(3).eq(&mut [1, 2]));
///
/// for t in digraph.iter_out_neighbors_mut(0) {
///     *t = (*t + 2) % 4;
/// }
///
/// assert!(digraph.iter_out_neighbors_mut(0).eq(&mut [3, 0]));
/// assert!(digraph.iter_out_neighbors_mut(1).eq(&mut [0, 2, 3]));
/// assert!(digraph.iter_out_neighbors_mut(2).eq(&mut [0, 1, 3]));
/// assert!(digraph.iter_out_neighbors_mut(3).eq(&mut [1, 2]));
/// ```
pub trait IterOutNeighborsMut {
    /// Returns an iterator to mutate the out-neighbors of a vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The tail vertex.
    fn iter_out_neighbors_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize>;
}

impl IterOutNeighborsMut for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self[s].iter_mut()
    }
}

impl IterOutNeighborsMut for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self[s].iter_mut()
    }
}

impl<const V: usize> IterOutNeighborsMut for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self[s].iter_mut()
    }
}

impl IterOutNeighborsMut for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self.get_mut(&s).unwrap().iter_mut()
    }
}

impl<H> IterOutNeighborsMut for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
        self.get_mut(&s).unwrap().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_iter_out_neighbors_mut_stable {
        ($digraph:expr) => {
            assert!($digraph.iter_out_neighbors_mut(0).eq(&mut [1, 2]));
            assert!($digraph.iter_out_neighbors_mut(1).eq(&mut [0, 2, 3]));
            assert!($digraph.iter_out_neighbors_mut(2).eq(&mut [0, 1, 3]));
            assert!($digraph.iter_out_neighbors_mut(3).eq(&mut [1, 2]));

            for t in $digraph.iter_out_neighbors_mut(0) {
                *t = (*t + 2) % 4;
            }

            assert!($digraph.iter_out_neighbors_mut(0).eq(&mut [3, 0]));
            assert!($digraph.iter_out_neighbors_mut(1).eq(&mut [0, 2, 3]));
            assert!($digraph.iter_out_neighbors_mut(2).eq(&mut [0, 1, 3]));
            assert!($digraph.iter_out_neighbors_mut(3).eq(&mut [1, 2]));
        };
    }

    #[test]
    fn vec_vec() {
        let digraph = &mut vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_out_neighbors_mut_stable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let digraph: &mut [Vec<usize>] =
            &mut [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_out_neighbors_mut_stable!(digraph);
    }

    #[test]
    fn arr_vec() {
        let digraph = &mut [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_out_neighbors_mut_stable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let digraph = &mut BTreeMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_iter_out_neighbors_mut_stable!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let digraph = &mut HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_iter_out_neighbors_mut_stable!(digraph);
    }
}
