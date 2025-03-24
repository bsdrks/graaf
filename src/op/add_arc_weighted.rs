//! Add an arc to an arc-weighted digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     ArcsWeighted,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<isize>::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(0, 2, 1);
//! digraph.add_arc_weighted(1, 2, -3);
//!
//! assert!(digraph
//!     .arcs_weighted()
//!     .eq([(0, 1, &2), (0, 2, &1), (1, 2, &-3)]));
//! ```

/// Add an arc to an arc-weighted digraph.
pub trait AddArcWeighted {
    /// The weight of an arc.
    type Weight;

    /// Add an arc from to the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    /// * `w`: The arc's weight.
    ///
    /// # Panics
    ///
    /// * Should panic if `u` equals `v`.
    /// * Should panic if `u` isn't in the digraph.
    /// * Should panic if `v` isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     ArcsWeighted,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::<isize>::empty(3);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(0, 2, 1);
    /// digraph.add_arc_weighted(1, 2, -3);
    ///
    /// assert!(digraph
    ///     .arcs_weighted()
    ///     .eq([(0, 1, &2), (0, 2, &1), (1, 2, &-3)]));
    /// ```
    fn add_arc_weighted(&mut self, u: usize, v: usize, w: Self::Weight);
}

/// `AddArcWeighted` proptests
#[macro_export]
macro_rules! proptest_add_arc_weighted {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::{
                proptest_strategy::arc,
                Degree,
            },
        };

        proptest! {
            #[test]
            fn add_arc_weighted_arc_weight(
                (u, v) in arc(25_usize),
                w in 1..25_usize
            ) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc_weighted(u, v, w);

                for x in digraph.vertices() {
                    for y in digraph.vertices() {
                        assert_eq!(
                            digraph.arc_weight(x, y),
                            (x == u && y == v).then_some(&w)
                        );
                    }
                }
            }

            #[test]
            fn add_arc_weighted_degree(
                (u, v) in arc(25_usize),
                w in 1..25_usize
            ) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc_weighted(u, v, w);

                for x in digraph.vertices() {
                    assert_eq!(
                        digraph.degree(x),
                        usize::from(x == u) + usize::from(x == v)
                    );
                }
            }

            #[test]
            fn add_arc_weighted_has_arc(
                (u, v) in arc(25_usize),
                w in 1..25_usize
            ) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc_weighted(u, v, w);

                assert!(digraph.has_arc(u, v));
            }

            #[test]
            fn add_arc_weighted_indegree(
                (u, v) in arc(25_usize),
                w in 1..25_usize
            ) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc_weighted(u, v, w);

                for u in digraph.vertices() {
                    assert_eq!(digraph.indegree(u), usize::from(u == v));
                }
            }

            #[test]
            fn add_arc_weighted_outdegree(
                (u, v) in arc(25_usize),
                w in 1..25_usize
            ) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc_weighted(u, v, w);

                for x in digraph.vertices() {
                    assert_eq!(digraph.outdegree(x), usize::from(x == u));
                }
            }

            #[test]
            fn add_arc_weighted_remove_arc(
                (u, v) in arc(25_usize),
                w in 1..25_usize
            ) {
                let d = <$type>::empty(100);
                let mut h = d.clone();

                h.add_arc_weighted(u, v, w);

                for x in d.vertices() {
                    for y in d.vertices() {
                        if x == u && y == v {
                            assert!(h.remove_arc(x, y));
                        } else {
                            assert!(!h.remove_arc(x, y));
                        }
                    }
                }

                assert_eq!(d, h);
            }
        }
    };
}
