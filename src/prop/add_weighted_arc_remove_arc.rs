//! Adding a weighted arc with [`AddWeightedArc`] and then removing it with
//! [`RemoveArc`] should keep the digraph unchanged.
//!
//! [`AddWeightedArc`]: crate::op::AddWeightedArc
//! [`RemoveArc`]: crate::op::RemoveArc

use crate::op::{
    AddWeightedArc,
    RemoveArc,
};

/// Returns whether adding a weighted arc with [`AddWeightedArc`] and then
/// removing it with [`RemoveArc`] keeps the digraph unchanged.
///
/// Types that implement [`AddWeightedArc`] and [`RemoveArc`] should ensure
/// that the property holds for every `digraph`, `s`, and `t` of the given
/// types.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The tail vertex.
/// * `t`: The head vertex.
/// * `w`: The weight of the arc.
///
/// [`AddWeightedArc`]: crate::op::AddWeightedArc
/// [`RemoveArc`]: crate::op::RemoveArc
pub fn add_weighted_arc_remove_arc<D, W>(digraph: &D, s: usize, t: usize, w: W) -> bool
where
    D: AddWeightedArc<W> + Clone + PartialEq + RemoveArc,
{
    let mut clone = digraph.clone();

    clone.add_weighted_arc(s, t, w);

    clone.remove_arc(s, t) && *digraph == clone
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
            prop::strategy::binop_vertices,
        },
        proptest::prelude::*,
        std::collections::{
            BTreeMap,
            HashMap,
        },
    };

    proptest! {
        #[test]
        fn vec_btree_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let digraph = Vec::<BTreeMap<usize, i32>>::empty(v);

            assert!(add_weighted_arc_remove_arc(&digraph, s, t, w));
        }

        #[test]
        fn vec_hash_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let digraph = Vec::<HashMap<usize, i32>>::empty(v);

            assert!(add_weighted_arc_remove_arc(&digraph, s, t, w));
        }
    }

    #[test]
    fn arr_btree_map() {
        let digraph = <[BTreeMap<usize, i32>; 3]>::empty();

        assert!(add_weighted_arc_remove_arc(&digraph, 0, 0, 0));
    }

    #[test]
    fn arr_hash_map() {
        let digraph = <[HashMap<usize, i32>; 3]>::empty();

        assert!(add_weighted_arc_remove_arc(&digraph, 0, 0, 0));
    }
}
