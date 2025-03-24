//! Add an arc to a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.arcs().eq([(0, 1), (0, 2), (2, 0)]));
//! ```

/// Add an arc to a digraph.
pub trait AddArc {
    /// Add an arc from `u` to `v` to the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
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
    ///     AddArc,
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.arcs().eq([(0, 1), (0, 2), (2, 0)]));
    /// ```
    fn add_arc(&mut self, u: usize, v: usize);
}

/// `AddArc` self-loop tests
#[macro_export]
macro_rules! test_add_arc_self_loop {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "u = 0 equals v = 0")]
        fn add_arc_u_equals_v() {
            <$type>::trivial().add_arc(0, 0);
        }
    };
}

/// `AddArc` out-of-bounds tests
#[macro_export]
macro_rules! test_add_arc_out_of_bounds {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "v = 1 isn't in the digraph")]
        fn add_arc_out_of_bounds_u() {
            <$type>::trivial().add_arc(0, 1);
        }

        #[test]
        #[should_panic(expected = "u = 1 isn't in the digraph")]
        fn add_arc_out_of_bounds_v() {
            <$type>::trivial().add_arc(1, 0);
        }
    };
}

/// `AddArc` proptests
#[macro_export]
macro_rules! proptest_add_arc {
    ($type:ty) => {
        use {
            $crate::{
                proptest_strategy::arc,
                Degree,
            },
            proptest::proptest,
        };

        proptest! {
            #[test]
            fn add_arc_degree((u, v) in arc(25_usize)) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph.vertices().all(|x| {
                    digraph.degree(x) == usize::from(x == u) + usize::from(x == v)
                }));
            }

            #[test]
            fn add_arc_has_arc((u, v) in arc(25_usize)) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph.has_arc(u, v));
            }

            #[test]
            fn add_arc_indegree((u, v) in arc(25_usize)) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph
                    .vertices()
                    .all(|x| digraph.indegree(x) == usize::from(x == v)));
            }

            #[test]
            fn add_arc_outdegree((u, v) in arc(25_usize)) {
                let mut digraph = <$type>::empty(100);

                digraph.add_arc(u, v);

                assert!(digraph
                    .vertices()
                    .all(|x| digraph.outdegree(x) == usize::from(x == u)));
            }

            #[test]
            fn add_arc_remove_arc((u, v) in arc(25_usize)) {
                let digraph = <$type>::empty(100);
                let mut h = digraph.clone();

                h.add_arc(u, v);

                assert!(digraph.vertices().all(|x| digraph
                    .vertices()
                    .all(|y| h.remove_arc(x, y) == (x == u && y == v))));

                assert_eq!(digraph, h);
            }

        }
    }
}
