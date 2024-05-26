//! Get the weight of a given arc
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::ArcWeight,
//!     std::collections::HashMap,
//! };
//!
//! let digraph = vec![
//!     HashMap::from([(1, 2), (2, 3)]),
//!     HashMap::from([(0, 4)]),
//!     HashMap::from([(0, 7), (1, 8)]),
//! ];
//!
//! assert_eq!(digraph.arc_weight(0, 0), None);
//! assert_eq!(digraph.arc_weight(0, 1), Some(&2));
//! assert_eq!(digraph.arc_weight(0, 2), Some(&3));
//! assert_eq!(digraph.arc_weight(1, 0), Some(&4));
//! assert_eq!(digraph.arc_weight(1, 1), None);
//! assert_eq!(digraph.arc_weight(2, 0), Some(&7));
//! assert_eq!(digraph.arc_weight(2, 1), Some(&8));
//! assert_eq!(digraph.arc_weight(2, 2), None);
//! ```

extern crate alloc;

use {
    alloc::collections::BTreeMap,
    core::hash::BuildHasher,
    std::collections::HashMap,
};

/// Get the weight of a given arc
///
/// # How can I implement `ArcWeight`?
///
/// Provide an implementation of `arc_weight` that returns the weight of the
/// arc from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::ArcWeight,
///     std::collections::HashMap,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashMap<usize, usize>>,
/// }
///
/// impl ArcWeight<usize> for Digraph {
///     fn arc_weight(&self, s: usize, t: usize) -> Option<&usize> {
///         self.arcs.get(s).and_then(|m| m.get(&t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::ArcWeight,
///     std::collections::HashMap,
/// };
///
/// let digraph = vec![
///     HashMap::from([(1, 2), (2, 3)]),
///     HashMap::from([(0, 4)]),
///     HashMap::from([(0, 7), (1, 8)]),
/// ];
///
/// assert_eq!(digraph.arc_weight(0, 0), None);
/// assert_eq!(digraph.arc_weight(0, 1), Some(&2));
/// assert_eq!(digraph.arc_weight(0, 2), Some(&3));
/// assert_eq!(digraph.arc_weight(1, 0), Some(&4));
/// assert_eq!(digraph.arc_weight(1, 1), None);
/// assert_eq!(digraph.arc_weight(2, 0), Some(&7));
/// assert_eq!(digraph.arc_weight(2, 1), Some(&8));
/// assert_eq!(digraph.arc_weight(2, 2), None);
/// ```
pub trait ArcWeight<W> {
    /// Returns the weight of the arc from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W>;
}

impl<W> ArcWeight<W> for Vec<BTreeMap<usize, W>> {
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(s).and_then(|m| m.get(&t))
    }
}

impl<W, H> ArcWeight<W> for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(s).and_then(|m| m.get(&t))
    }
}

impl<W> ArcWeight<W> for [BTreeMap<usize, W>] {
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(s).and_then(|m| m.get(&t))
    }
}

impl<W, H> ArcWeight<W> for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(s).and_then(|m| m.get(&t))
    }
}

impl<const V: usize, W> ArcWeight<W> for [BTreeMap<usize, W>; V] {
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(s).and_then(|m| m.get(&t))
    }
}

impl<const V: usize, W, H> ArcWeight<W> for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(s).and_then(|m| m.get(&t))
    }
}

impl<W> ArcWeight<W> for BTreeMap<usize, BTreeMap<usize, W>> {
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(&s).and_then(|m| m.get(&t))
    }
}

impl<W, H> ArcWeight<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
        self.get(&s).and_then(|m| m.get(&t))
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
            op::AddWeightedArc,
        },
    };

    macro_rules! test_arc_weight {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 3);
            $digraph.add_weighted_arc(1, 0, 4);
            $digraph.add_weighted_arc(2, 0, 7);
            $digraph.add_weighted_arc(2, 1, 8);

            assert_eq!($digraph.arc_weight(0, 1), Some(&2));
            assert_eq!($digraph.arc_weight(0, 2), Some(&3));
            assert_eq!($digraph.arc_weight(1, 0), Some(&4));
            assert_eq!($digraph.arc_weight(2, 0), Some(&7));
            assert_eq!($digraph.arc_weight(2, 1), Some(&8));
        };
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(3);

        test_arc_weight!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(3);

        test_arc_weight!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(3);

        test_arc_weight!(digraph.as_mut_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(3);

        test_arc_weight!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, i32>; 3]>::empty();

        test_arc_weight!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, i32>; 3]>::empty();

        test_arc_weight!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, i32>>::empty(3);

        test_arc_weight!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, i32>>::empty(3);

        test_arc_weight!(digraph);
    }
}
