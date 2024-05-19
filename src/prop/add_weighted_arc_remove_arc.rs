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

    let _ = clone.remove_arc(s, t);

    *digraph == clone
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::prop::strategy::binop_vertices,
        alloc::collections::BTreeMap,
        proptest::prelude::*,
        std::collections::HashMap,
    };

    proptest! {
        #[test]
        fn vec_btree_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let digraph = vec![BTreeMap::new(); v];

            assert!(add_weighted_arc_remove_arc(&digraph, s, t, w));
        }

        #[test]
        fn vec_hash_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let digraph = vec![HashMap::new(); v];

            assert!(add_weighted_arc_remove_arc(&digraph, s, t, w));
        }
    }

    #[test]
    fn arr_btree_map() {
        let digraph = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];

        assert!(add_weighted_arc_remove_arc(&digraph, 0, 0, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 0, 1, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 0, 2, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 1, 0, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 1, 1, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 1, 2, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 2, 0, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 2, 1, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 2, 2, 0));
    }

    #[test]
    fn arr_hash_map() {
        let digraph = [HashMap::new(), HashMap::new(), HashMap::new()];

        assert!(add_weighted_arc_remove_arc(&digraph, 0, 0, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 0, 1, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 0, 2, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 1, 0, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 1, 1, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 1, 2, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 2, 0, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 2, 1, 0));
        assert!(add_weighted_arc_remove_arc(&digraph, 2, 2, 0));
    }
}
