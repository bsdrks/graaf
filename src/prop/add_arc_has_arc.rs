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
/// property holds for every `digraph`, `s`, and `t` of the given types.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
///
/// [`AddArc`]: crate::op::AddArc
/// [`HasArc`]: crate::op::HasArc
pub fn add_arc_has_arc<D>(digraph: &mut D, s: usize, t: usize) -> bool
where
    D: AddArc + HasArc + ?Sized,
{
    digraph.add_arc(s, t);

    digraph.has_arc(s, t)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::Empty,
            prop::strategy::binop_vertices,
        },
        proptest::prelude::*,
        std::collections::{
            BTreeMap,
            BTreeSet,
            HashMap,
            HashSet,
        },
    };

    proptest! {
        #[test]
        fn vec_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = vec![BTreeSet::new(); v];

            assert!(add_arc_has_arc(&mut digraph, s, t));
        }

        #[test]
        fn vec_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = vec![HashSet::new(); v];

            assert!(add_arc_has_arc(&mut digraph, s, t));
        }

        #[test]
        fn slice_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let digraph = &mut vec![BTreeSet::new(); v][..];

            assert!(add_arc_has_arc(digraph, s, t));
        }

        #[test]
        fn slice_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = Vec::<HashSet<usize>>::empty(v);

            assert!(add_arc_has_arc(digraph.as_mut_slice(), s, t));
        }

        #[test]
        fn btree_map_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = (0..v)
                .map(|v| (v, BTreeSet::new()))
                .collect::<BTreeMap<_, _>>();

            assert!(add_arc_has_arc(&mut digraph, s, t));
        }

        #[test]
        fn hash_map_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = (0..v)
                .map(|v| (v, HashSet::new()))
                .collect::<HashMap<_, _>>();

            assert!(add_arc_has_arc(&mut digraph, s, t));
        }
    }

    #[test]
    fn arr_btree_set() {
        let digraph = &mut [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        assert!(add_arc_has_arc(digraph, 0, 0));
        assert!(add_arc_has_arc(digraph, 0, 1));
        assert!(add_arc_has_arc(digraph, 0, 2));
        assert!(add_arc_has_arc(digraph, 1, 0));
        assert!(add_arc_has_arc(digraph, 1, 1));
        assert!(add_arc_has_arc(digraph, 1, 2));
        assert!(add_arc_has_arc(digraph, 2, 0));
        assert!(add_arc_has_arc(digraph, 2, 1));
        assert!(add_arc_has_arc(digraph, 2, 2));
    }

    #[test]
    fn arr_hash_set() {
        let digraph = &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        assert!(add_arc_has_arc(digraph, 0, 0));
        assert!(add_arc_has_arc(digraph, 0, 1));
        assert!(add_arc_has_arc(digraph, 0, 2));
        assert!(add_arc_has_arc(digraph, 1, 0));
        assert!(add_arc_has_arc(digraph, 1, 1));
        assert!(add_arc_has_arc(digraph, 1, 2));
        assert!(add_arc_has_arc(digraph, 2, 0));
        assert!(add_arc_has_arc(digraph, 2, 1));
        assert!(add_arc_has_arc(digraph, 2, 2));
    }

    #[cfg(feature = "adjacency_matrix")]
    #[test]
    fn adjacency_matrix() {
        use crate::repr::AdjacencyMatrix;

        let digraph = &mut AdjacencyMatrix::<3>::new();

        assert!(add_arc_has_arc(digraph, 0, 0));
        assert!(add_arc_has_arc(digraph, 0, 1));
        assert!(add_arc_has_arc(digraph, 0, 2));
        assert!(add_arc_has_arc(digraph, 1, 0));
        assert!(add_arc_has_arc(digraph, 1, 1));
        assert!(add_arc_has_arc(digraph, 1, 2));
        assert!(add_arc_has_arc(digraph, 2, 0));
        assert!(add_arc_has_arc(digraph, 2, 1));
        assert!(add_arc_has_arc(digraph, 2, 2));
    }
}
