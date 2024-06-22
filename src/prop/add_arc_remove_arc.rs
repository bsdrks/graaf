//! Adding an arc with [`AddArc`] and then removing it with [`RemoveArc`]
//! should keep the digraph unchanged.
//!
//! [`AddArc`]: crate::op::AddArc
//! [`RemoveArc`]: crate::op::RemoveArc

use crate::op::{
    AddArc,
    RemoveArc,
};

/// Returns whether adding an arc with [`AddArc`] and then removing it with
/// [`RemoveArc`] keeps the digraph unchanged.
///
/// Types that implement [`AddArc`] and [`RemoveArc`] should ensure that the
/// property holds for every `digraph`, `s`, and `t` of the given types.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
/// * `t`: The target vertex.
///
/// [`AddArc`]: crate::op::AddArc
/// [`RemoveArc`]: crate::op::RemoveArc
pub fn add_arc_remove_arc<D>(digraph: &D, s: usize, t: usize) -> bool
where
    D: AddArc + Clone + PartialEq + RemoveArc,
{
    let mut clone = digraph.clone();

    clone.add_arc(s, t);

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
            BTreeSet,
            HashSet,
        },
    };

    proptest! {
        #[test]
        fn vec_btree_set((v, s, t) in binop_vertices(1, 100)) {
            let digraph = Vec::<BTreeSet<usize>>::empty(v);

            assert!(add_arc_remove_arc(&digraph, s, t));
        }

        #[test]
        fn vec_hash_set((v, s, t) in binop_vertices(1, 100)) {
            let digraph = Vec::<HashSet<usize>>::empty(v);

            assert!(add_arc_remove_arc(&digraph, s, t));
        }
    }

    #[test]
    fn arr_btree_set() {
        let digraph = <[BTreeSet<usize>; 3]>::empty();

        assert!(add_arc_remove_arc(&digraph, 0, 0));
    }

    #[test]
    fn arr_hash_set() {
        let digraph = <[HashSet<usize>; 3]>::empty();

        assert!(add_arc_remove_arc(&digraph, 0, 0));
    }

    #[cfg(feature = "adjacency_matrix")]
    #[test]
    fn adjacency_matrix() {
        use crate::repr::AdjacencyMatrix;

        let digraph = AdjacencyMatrix::<3>::new();

        assert!(add_arc_remove_arc(&digraph, 0, 0));
    }
}
