//! Adding an arc with [`AddArc`] should be reflected by [`HasArc`].
//!
//! [`AddArc`]: crate::op::AddArc
//! [`HasArc`]: crate::op::HasArc

use crate::op::{
    AddArc,
    HasArc,
};

/// Returns whether adding an arc with [`AddArc`] is reflected by [`HasArc`].
///
/// Types that implement [`AddArc`] and [`HasArc`] should ensure that this
/// property holds for every `graph`, `s`, and `t` of the given types.
///
/// # Arguments
///
/// * `graph`: The graph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
///
/// [`AddArc`]: crate::op::AddArc
/// [`HasArc`]: crate::op::HasArc
pub fn add_arc_has_arc<G>(graph: &mut G, s: usize, t: usize) -> bool
where
    G: AddArc + HasArc + ?Sized,
{
    graph.add_arc(s, t);

    graph.has_arc(s, t)
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::prop::strategy::binop_vertices,
        alloc::collections::{
            BTreeMap,
            BTreeSet,
        },
        proptest::prelude::*,
        std::collections::{
            HashMap,
            HashSet,
        },
    };

    proptest! {
        #[test]
        fn vec_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let mut graph = vec![BTreeSet::new(); v];

            assert!(add_arc_has_arc(&mut graph, s, t));
        }

        #[test]
        fn vec_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let mut graph = vec![HashSet::new(); v];

            assert!(add_arc_has_arc(&mut graph, s, t));
        }

        #[test]
        fn slice_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let graph = &mut vec![BTreeSet::new(); v][..];

            assert!(add_arc_has_arc(graph, s, t));
        }

        #[test]
        fn slice_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let graph = &mut vec![HashSet::new(); v][..];

            assert!(add_arc_has_arc(graph, s, t));
        }

        #[test]
        fn btree_map_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let mut graph = (0..v)
                .map(|v| (v, BTreeSet::new()))
                .collect::<BTreeMap<_, _>>();

            assert!(add_arc_has_arc(&mut graph, s, t));
        }

        #[test]
        fn hash_map_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let mut graph = (0..v)
                .map(|v| (v, HashSet::new()))
                .collect::<HashMap<_, _>>();

            assert!(add_arc_has_arc(&mut graph, s, t));
        }
    }

    #[test]
    fn arr_btree_set() {
        let graph = &mut [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        assert!(add_arc_has_arc(graph, 0, 0));
        assert!(add_arc_has_arc(graph, 0, 1));
        assert!(add_arc_has_arc(graph, 0, 2));
        assert!(add_arc_has_arc(graph, 1, 0));
        assert!(add_arc_has_arc(graph, 1, 1));
        assert!(add_arc_has_arc(graph, 1, 2));
        assert!(add_arc_has_arc(graph, 2, 0));
        assert!(add_arc_has_arc(graph, 2, 1));
        assert!(add_arc_has_arc(graph, 2, 2));
    }

    #[test]
    fn arr_hash_set() {
        let graph = &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        assert!(add_arc_has_arc(graph, 0, 0));
        assert!(add_arc_has_arc(graph, 0, 1));
        assert!(add_arc_has_arc(graph, 0, 2));
        assert!(add_arc_has_arc(graph, 1, 0));
        assert!(add_arc_has_arc(graph, 1, 1));
        assert!(add_arc_has_arc(graph, 1, 2));
        assert!(add_arc_has_arc(graph, 2, 0));
        assert!(add_arc_has_arc(graph, 2, 1));
        assert!(add_arc_has_arc(graph, 2, 2));
    }

    #[cfg(feature = "adjacency_matrix")]
    #[test]
    fn adjacency_matrix() {
        use crate::repr::AdjacencyMatrix;

        let graph = &mut AdjacencyMatrix::<3>::new();

        assert!(add_arc_has_arc(graph, 0, 0));
        assert!(add_arc_has_arc(graph, 0, 1));
        assert!(add_arc_has_arc(graph, 0, 2));
        assert!(add_arc_has_arc(graph, 1, 0));
        assert!(add_arc_has_arc(graph, 1, 1));
        assert!(add_arc_has_arc(graph, 1, 2));
        assert!(add_arc_has_arc(graph, 2, 0));
        assert!(add_arc_has_arc(graph, 2, 1));
        assert!(add_arc_has_arc(graph, 2, 2));
    }
}
