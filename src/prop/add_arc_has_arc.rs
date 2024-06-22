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
            gen::{
                Empty,
                EmptyConst,
            },
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
            let mut digraph = Vec::<BTreeSet<usize>>::empty(v);

            assert!(add_arc_has_arc(&mut digraph, s, t));
        }

        #[test]
        fn vec_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = Vec::<HashSet<usize>>::empty(v);

            assert!(add_arc_has_arc(&mut digraph, s, t));
        }

        #[test]
        fn slice_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = Vec::<BTreeSet<usize>>::empty(v);

            assert!(add_arc_has_arc(digraph.as_mut_slice(), s, t));
        }

        #[test]
        fn slice_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = Vec::<HashSet<usize>>::empty(v);

            assert!(add_arc_has_arc(digraph.as_mut_slice(), s, t));
        }

        #[test]
        fn btree_map_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(v);

            assert!(add_arc_has_arc(&mut digraph, s, t));
        }

        #[test]
        fn hash_map_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let mut digraph = HashMap::<usize, HashSet<usize>>::empty(v);

            assert!(add_arc_has_arc(&mut digraph, s, t));
        }
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        assert!(add_arc_has_arc(&mut digraph, 0, 0));
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        assert!(add_arc_has_arc(&mut digraph, 0, 0));
    }

    #[cfg(feature = "adjacency_matrix")]
    #[test]
    fn adjacency_matrix() {
        use crate::repr::AdjacencyMatrix;

        let digraph = &mut AdjacencyMatrix::<3>::new();

        assert!(add_arc_has_arc(digraph, 0, 0));
    }
}
