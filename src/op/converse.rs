//! Generate the converse of a digraph.
//!
//! The converse of a digraph is a digraph with all arcs reversed.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         gen::CycleConst,
//!         op::Converse,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let digraph: [BTreeSet<usize>; 4] = <[BTreeSet<usize>; 4]>::cycle();
//!
//! assert_eq!(
//!     digraph,
//!     [
//!         BTreeSet::from([1]),
//!         BTreeSet::from([2]),
//!         BTreeSet::from([3]),
//!         BTreeSet::from([0]),
//!     ]
//! );
//!
//! let converse = digraph.converse();
//!
//! assert_eq!(
//!     converse,
//!     [
//!         BTreeSet::from([3]),
//!         BTreeSet::from([0]),
//!         BTreeSet::from([1]),
//!         BTreeSet::from([2]),
//!     ]
//! );
//! ```

use {
    crate::{
        gen::{
            Empty,
            EmptyConst,
        },
        op::{
            AddArc,
            AddWeightedArc,
            IterArcs,
            IterWeightedArcs,
            Size,
        },
    },
    core::{
        array::from_fn,
        hash::{
            BuildHasher,
            Hash,
        },
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

/// Generate the converse of a digraph.
///
/// # How do I implement `Converse`?
///
/// Provide an implementation of `converse` that returns a digraph with all arcs
/// reversed.
///
/// ```
/// use {
///     graaf::{
///         gen::Empty,
///         op::{
///             AddArc,
///             Converse,
///             IterArcs,
///             Size,
///         },
///     },
///     std::collections::HashSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl Converse for Digraph {
///     fn converse(&self) -> Self {
///         let v = self.arcs.size();
///         let mut converse = Vec::<HashSet<usize>>::empty(v);
///
///         for (s, t) in self.arcs.iter_arcs() {
///             converse.add_arc(t, s);
///         }
///
///         Self { arcs: converse }
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([0])],
/// };
///
/// let converse = digraph.converse();
///
/// assert_eq!(
///     converse.arcs,
///     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1]),],
/// );
/// ```
pub trait Converse {
    /// Generates the converse of the digraph
    #[must_use]
    fn converse(&self) -> Self;
}

macro_rules! impl_converse {
    () => {
        fn converse(&self) -> Self {
            let v = self.size();
            let mut converse = Self::empty(v);

            for (s, t) in self.iter_arcs() {
                converse.add_arc(t, s);
            }

            converse
        }
    };
}

macro_rules! impl_converse_const {
    () => {
        fn converse(&self) -> Self {
            let mut converse = Self::empty();

            for (s, t) in self.iter_arcs() {
                converse.add_arc(t, s);
            }

            converse
        }
    };
}

macro_rules! impl_converse_weighted {
    () => {
        fn converse(&self) -> Self {
            let v = self.size();
            let mut converse = Self::empty(v);

            for (s, t, w) in self.iter_weighted_arcs() {
                converse.add_weighted_arc(t, s, *w);
            }

            converse
        }
    };
}

macro_rules! impl_converse_weighted_const {
    () => {
        fn converse(&self) -> Self {
            let mut converse = Self::empty();

            for (s, t, w) in self.iter_weighted_arcs() {
                converse.add_weighted_arc(t, s, *w);
            }

            converse
        }
    };
}

impl Converse for Vec<Vec<usize>> {
    impl_converse!();
}

impl Converse for Vec<BTreeSet<usize>> {
    impl_converse!();
}

impl<H> Converse for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Clone + Default,
{
    impl_converse!();
}

impl<const V: usize> Converse for [Vec<usize>; V] {
    impl_converse_const!();
}

impl<const V: usize> Converse for [BTreeSet<usize>; V] {
    impl_converse_const!();
}

impl<const V: usize, H> Converse for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    impl_converse_const!();
}

impl Converse for BTreeMap<usize, Vec<usize>> {
    impl_converse!();
}

impl Converse for BTreeMap<usize, BTreeSet<usize>> {
    impl_converse!();
}

impl<H> Converse for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    impl_converse!();
}

impl<H> Converse for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    impl_converse!();
}

impl<W> Converse for Vec<Vec<(usize, W)>>
where
    W: Copy,
{
    impl_converse_weighted!();
}

impl<W> Converse for Vec<BTreeSet<(usize, W)>>
where
    W: Copy + Ord,
{
    impl_converse_weighted!();
}

impl<W> Converse for Vec<BTreeMap<usize, W>>
where
    W: Copy + Ord,
{
    impl_converse_weighted!();
}

impl<W, H> Converse for Vec<HashSet<(usize, W), H>>
where
    W: Copy + Eq + Hash,
    H: BuildHasher + Clone + Default,
{
    impl_converse_weighted!();
}

impl<W, H> Converse for Vec<HashMap<usize, W, H>>
where
    W: Copy,
    H: BuildHasher + Clone + Default,
{
    impl_converse_weighted!();
}

impl<const V: usize, W> Converse for [Vec<(usize, W)>; V]
where
    W: Copy,
{
    impl_converse_weighted_const!();
}

impl<const V: usize, W> Converse for [BTreeSet<(usize, W)>; V]
where
    W: Copy + Ord,
{
    impl_converse_weighted_const!();
}

impl<const V: usize, W> Converse for [BTreeMap<usize, W>; V]
where
    W: Copy + Default + Ord,
{
    impl_converse_weighted_const!();
}

impl<const V: usize, W, H> Converse for [HashSet<(usize, W), H>; V]
where
    W: Copy + Default + Eq + Hash,
    H: BuildHasher + Default,
{
    impl_converse_weighted_const!();
}

impl<const V: usize, W, H> Converse for [HashMap<usize, W, H>; V]
where
    W: Copy + Default,
    H: BuildHasher + Default,
{
    impl_converse_weighted_const!();
}

impl<W> Converse for BTreeMap<usize, Vec<(usize, W)>>
where
    W: Copy,
{
    impl_converse_weighted!();
}

impl<W> Converse for BTreeMap<usize, BTreeSet<(usize, W)>>
where
    W: Copy + Ord,
{
    impl_converse_weighted!();
}

impl<W> Converse for BTreeMap<usize, BTreeMap<usize, W>>
where
    W: Copy + Ord,
{
    impl_converse_weighted!();
}

impl<W, H> Converse for HashMap<usize, Vec<(usize, W)>, H>
where
    W: Copy,
    H: BuildHasher + Default,
{
    impl_converse_weighted!();
}

impl<W, H> Converse for HashMap<usize, HashSet<(usize, W), H>, H>
where
    W: Copy + Eq + Hash,
    H: BuildHasher + Default,
{
    impl_converse_weighted!();
}

impl<W, H> Converse for HashMap<usize, HashMap<usize, W, H>, H>
where
    W: Copy,
    H: BuildHasher + Default,
{
    impl_converse_weighted!();
}

impl Converse for Vec<(usize, usize)> {
    impl_converse!();
}

impl<const V: usize> Converse for [(usize, usize); V] {
    fn converse(&self) -> Self {
        from_fn(|i| (self[i].1, self[i].0))
    }
}

impl Converse for BTreeSet<(usize, usize)> {
    impl_converse!();
}

impl<H> Converse for HashSet<(usize, usize), H>
where
    H: BuildHasher + Default,
{
    impl_converse!();
}

impl<W> Converse for Vec<(usize, usize, W)>
where
    W: Copy,
{
    impl_converse_weighted!();
}

impl<const V: usize, W> Converse for [(usize, usize, W); V]
where
    W: Copy,
{
    fn converse(&self) -> Self {
        from_fn(|i| (self[i].1, self[i].0, self[i].2))
    }
}

impl<W> Converse for BTreeSet<(usize, usize, W)>
where
    W: Copy + Ord,
{
    impl_converse_weighted!();
}

impl<W, H> Converse for HashSet<(usize, usize, W), H>
where
    W: Copy + Eq + Hash,
    H: BuildHasher + Default,
{
    impl_converse_weighted!();
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! setup {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 0);
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 2, 2);
            $digraph.add_weighted_arc(1, 0, 3);
        };
    }

    macro_rules! test_unweighted {
        ($digraph:expr) => {
            let converse = $digraph.converse();
            let mut arcs = converse.iter_arcs();

            assert!(matches!(arcs.next(), Some((0, 1) | (1 | 2, 0))));
            assert!(matches!(arcs.next(), Some((0, 1) | (1 | 2, 0))));
            assert!(matches!(arcs.next(), Some((0, 1) | (1 | 2, 0))));
            assert_eq!(arcs.next(), None);

            assert_eq!(
                $digraph.iter_arcs().collect::<BTreeSet<_>>(),
                converse.converse().iter_arcs().collect::<BTreeSet<_>>()
            );
        };
    }

    macro_rules! test_weighted {
        ($digraph:expr) => {
            let converse = $digraph.converse();
            let mut arcs = converse.iter_weighted_arcs();

            assert!(matches!(
                arcs.next(),
                Some((0, 1, 3) | (1, 0, 1) | (2, 0, 2))
            ));

            assert!(matches!(
                arcs.next(),
                Some((0, 1, 3) | (1, 0, 1) | (2, 0, 2))
            ));

            assert!(matches!(
                arcs.next(),
                Some((0, 1, 3) | (1, 0, 1) | (2, 0, 2))
            ));

            assert_eq!(arcs.next(), None);

            assert_eq!(
                $digraph.iter_weighted_arcs().collect::<BTreeSet<_>>(),
                converse
                    .converse()
                    .iter_weighted_arcs()
                    .collect::<BTreeSet<_>>()
            );
        };
    }

    #[test]
    fn vec_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn arr_vec() {
        let mut digraph: [Vec<usize>; 3] = <[Vec<usize>; 3]>::empty();

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph: [BTreeSet<usize>; 3] = <[BTreeSet<usize>; 3]>::empty();

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph: [HashSet<usize>; 3] = <[HashSet<usize>; 3]>::empty();

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<usize>>::empty(3);

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(3);

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(3);

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn vec_vec_weighted() {
        let mut digraph = Vec::<Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let mut digraph = Vec::<BTreeSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn vec_btree_map_weighted() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let mut digraph = Vec::<HashSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn vec_hash_map_weighted() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn arr_vec_weighted() {
        let mut digraph: [Vec<(usize, usize)>; 3] = <[Vec<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let mut digraph: [BTreeSet<(usize, usize)>; 3] = <[BTreeSet<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn arr_btree_map_weighted() {
        let mut digraph: [BTreeMap<usize, usize>; 3] = <[BTreeMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let mut digraph: [HashSet<(usize, usize)>; 3] = <[HashSet<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn arr_hash_map_weighted() {
        let mut digraph: [HashMap<usize, usize>; 3] = <[HashMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn btree_map_vec_weighted() {
        let mut digraph = BTreeMap::<usize, Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn btree_map_btree_set_weighted() {
        let mut digraph = BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn btree_map_btree_map_weighted() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn hash_map_vec_weighted() {
        let mut digraph = HashMap::<usize, Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn hash_map_hash_set_weighted() {
        let mut digraph = HashMap::<usize, HashSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn hash_map_hash_map_weighted() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn vec_tuple() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn arr_tuple() {
        let digraph: [(usize, usize); 3] = [(0, 1), (0, 2), (1, 0)];

        test_unweighted!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize)>::new();

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::<(usize, usize)>::new();

        setup!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn vec_tuple_weighted() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn arr_tuple_weighted() {
        let digraph: [(usize, usize, usize); 3] = [(0, 1, 1), (0, 2, 2), (1, 0, 3)];

        test_weighted!(digraph);
    }

    #[test]
    fn btree_set_tuple_weighted() {
        let mut digraph = BTreeSet::<(usize, usize, usize)>::new();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn hash_set_tuple_weighted() {
        let mut digraph = HashSet::<(usize, usize, usize)>::new();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }
}
