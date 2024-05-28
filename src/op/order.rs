//! Count the vertices in a digraph.
//!
//! # Example
//!
//! ```
//! use graaf::op::Order;
//!
//! let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(digraph.order(), 4);
//! ```

extern crate alloc;

use {
    alloc::collections::{
        BTreeMap,
        BTreeSet,
    },
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// Count the vertices in a digraph.
///
/// # How can I implement `Order`?
///
/// Provides an implementation of `order` that returns the number
/// of vertices in the digraph.
///
/// ```
/// use graaf::op::Order;
///
/// struct Digraph {
///     vertices: Vec<usize>,
/// }
///
/// impl Order for Digraph {
///     fn order(&self) -> usize {
///         self.vertices.len()
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use graaf::op::Order;
///
/// let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(digraph.order(), 4);
/// ```
pub trait Order {
    /// Count all vertices.
    fn order(&self) -> usize;
}

impl Order for Vec<Vec<usize>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for Vec<Vec<(usize, W)>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl Order for Vec<BTreeSet<usize>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for Vec<BTreeSet<(usize, W)>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<H> Order for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W, H> Order for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for Vec<BTreeMap<usize, W>> {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W, H> Order for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl Order for [Vec<usize>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for [Vec<(usize, W)>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl Order for [BTreeSet<usize>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for [BTreeSet<(usize, W)>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<H> Order for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W, H> Order for [HashSet<(usize, W), H>]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W> Order for [BTreeMap<usize, W>] {
    fn order(&self) -> usize {
        self.len()
    }
}

impl<W, H> Order for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        self.len()
    }
}

impl<const V: usize> Order for [Vec<usize>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W> Order for [Vec<(usize, W)>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize> Order for [BTreeSet<usize>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W> Order for [BTreeSet<(usize, W)>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, H> Order for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W, H> Order for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W> Order for [BTreeMap<usize, W>; V] {
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize, W, H> Order for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn order(&self) -> usize {
        V
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
            op::{
                AddArc,
                AddWeightedArc,
            },
        },
    };

    macro_rules! setup_unweighted {
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

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 3);
            $digraph.add_weighted_arc(1, 0, 4);
            $digraph.add_weighted_arc(1, 2, 5);
            $digraph.add_weighted_arc(1, 3, 6);
            $digraph.add_weighted_arc(2, 0, 7);
            $digraph.add_weighted_arc(2, 1, 8);
            $digraph.add_weighted_arc(2, 3, 9);
            $digraph.add_weighted_arc(3, 1, 10);
            $digraph.add_weighted_arc(3, 2, 11);
        };
    }

    #[test]
    fn vec_vec_unweighted() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn slice_vec_unweighted() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.as_slice().order(), 4);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.as_slice().order(), 4);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        assert_eq!(digraph.as_slice().order(), 4);
    }

    #[test]
    fn arr_vec_unweighted() {
        let mut digraph = <[Vec<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let mut digraph = <[BTreeSet<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let mut digraph = <[HashSet<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_vec_weighted() {
        let mut digraph = Vec::<Vec<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let mut digraph = Vec::<BTreeSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let mut digraph = Vec::<HashSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn slice_vec_weighted() {
        let mut digraph = Vec::<Vec<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.as_slice().order(), 4);
    }

    #[test]
    fn slice_btree_set_weighted() {
        let mut digraph = Vec::<BTreeSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.as_slice().order(), 4);
    }

    #[test]
    fn slice_hash_set_weighted() {
        let mut digraph = Vec::<HashSet<(usize, i32)>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.as_slice().order(), 4);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.as_slice().order(), 4);
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        assert_eq!(digraph.as_slice().order(), 4);
    }

    #[test]
    fn arr_vec_weighted() {
        let mut digraph = <[Vec<(usize, i32)>; 4]>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let mut digraph = <[BTreeSet<(usize, i32)>; 4]>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let mut digraph = <[HashSet<(usize, i32)>; 4]>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, i32>; 4]>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, i32>; 4]>::empty();

        setup_weighted!(digraph);
        assert_eq!(digraph.order(), 4);
    }
}
