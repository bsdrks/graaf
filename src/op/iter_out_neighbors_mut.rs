//! Return an iterator to mutate the out-neighbors of a vertex in
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

/// Return an iterator to mutate the out-neighbors of a vertex in
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
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl IterOutNeighborsMut for Digraph {
///     fn iter_out_neighbors_mut(&mut self, s: usize) -> impl Iterator<Item = &mut usize> {
///         self.arcs[s].iter_mut()
///     }
/// }
///
/// let mut digraph = Digraph {
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
    use {
        super::*,
        crate::{
            gen::{
                Empty,
                EmptyConst,
            },
            op::AddArc,
        },
    };

    macro_rules! setup {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 0);
            $digraph.add_arc(1, 2);
            $digraph.add_arc(1, 3);
            $digraph.add_arc(2, 0);
            $digraph.add_arc(2, 1);
            $digraph.add_arc(2, 3);
            $digraph.add_arc(3, 1);
            $digraph.add_arc(3, 2);
        };
    }

    macro_rules! test_iter_out_neighbors_mut {
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
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup!(digraph);
        test_iter_out_neighbors_mut!(digraph);
    }

    #[test]
    fn slice_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup!(digraph);
        test_iter_out_neighbors_mut!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_vec() {
        let mut digraph = <[Vec<usize>; 4]>::empty();

        setup!(digraph);
        test_iter_out_neighbors_mut!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<usize>>::empty(4);

        setup!(digraph);
        test_iter_out_neighbors_mut!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(4);

        setup!(digraph);
        test_iter_out_neighbors_mut!(digraph);
    }
}
