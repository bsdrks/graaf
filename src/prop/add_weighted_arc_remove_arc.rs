//! Adding a weighted arc with [`AddWeightedArc`] and then removing it with
//! [`RemoveArc`] should keep the graph unchanged.
//!
//! [`AddWeightedArc`]: crate::op::AddWeightedArc
//! [`RemoveArc`]: crate::op::RemoveArc

use crate::op::{
    AddWeightedArc,
    RemoveArc,
};

/// Returns where adding a weighted arc with [`AddWeightedArc`] and then
/// removing it with [`RemoveArc`] keeps the graph unchanged.
///
/// Types that implement [`AddWeightedArc`] and [`RemoveArc`] should ensure
/// that the property holds for every `graph`, `s`, and `t` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
/// * `w`: The weight of the arc.
///
/// [`AddWeightedArc`]: crate::op::AddWeightedArc
/// [`RemoveArc`]: crate::op::RemoveArc
pub fn add_weighted_arc_remove_arc<G, W>(graph: &G, s: usize, t: usize, w: W) -> bool
where
    G: AddWeightedArc<W> + Clone + PartialEq + RemoveArc,
{
    let mut clone = graph.clone();

    clone.add_weighted_arc(s, t, w);

    let _ = clone.remove_arc(s, t);

    *graph == clone
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
            let graph = vec![BTreeMap::new(); v];

            assert!(add_weighted_arc_remove_arc(&graph, s, t, w));
        }

        #[test]
        fn vec_hash_map((v, s, t) in binop_vertices(1, 100), w in -100..100_i32) {
            let graph = vec![HashMap::new(); v];

            assert!(add_weighted_arc_remove_arc(&graph, s, t, w));
        }
    }

    #[test]
    fn arr_btree_map() {
        let graph = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];

        assert!(add_weighted_arc_remove_arc(&graph, 0, 0, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 0, 1, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 0, 2, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 1, 0, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 1, 1, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 1, 2, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 2, 0, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 2, 1, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 2, 2, 0));
    }

    #[test]
    fn arr_hash_map() {
        let graph = [HashMap::new(), HashMap::new(), HashMap::new()];

        assert!(add_weighted_arc_remove_arc(&graph, 0, 0, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 0, 1, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 0, 2, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 1, 0, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 1, 1, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 1, 2, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 2, 0, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 2, 1, 0));
        assert!(add_weighted_arc_remove_arc(&graph, 2, 2, 0));
    }
}
