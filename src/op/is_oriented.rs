//! Check whether a digraph is oriented.
//!
//! An oriented digraph has no cycle of length 2.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     IsOriented,
//! };
//!
//! assert!(!AdjacencyList::circuit(2).is_oriented());
//! assert!(AdjacencyList::circuit(3).is_oriented());
//! ```

use crate::{
    Arcs,
    HasArc,
};

/// Check whether a digraph is oriented.
pub trait IsOriented {
    /// Check whether the digraph is oriented.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     IsOriented,
    /// };
    ///
    /// assert!(!AdjacencyList::circuit(2).is_oriented());
    /// assert!(AdjacencyList::circuit(3).is_oriented());
    /// ```
    #[must_use]
    fn is_oriented(&self) -> bool;
}

impl<D> IsOriented for D
where
    D: Arcs + HasArc,
{
    fn is_oriented(&self) -> bool {
        self.arcs().all(|(u, v)| !self.has_arc(v, u))
    }
}

/// `IsOriented` tests
#[macro_export]
macro_rules! test_is_oriented {
    ($fixture:path) => {
        use $fixture::{
            bang_jensen_196,
            bang_jensen_34,
            bang_jensen_94,
            kattis_builddeps,
            kattis_cantinaofbabel_1,
            kattis_cantinaofbabel_2,
            kattis_escapewallmaria_1,
            kattis_escapewallmaria_2,
            kattis_escapewallmaria_3,
        };

        #[test]
        fn is_oriented_bang_jensen_196() {
            assert!(!bang_jensen_196().is_oriented());
        }

        #[test]
        fn is_oriented_bang_jensen_34() {
            assert!(bang_jensen_34().is_oriented());
        }

        #[test]
        fn is_oriented_bang_jensen_94() {
            assert!(bang_jensen_94().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_builddeps() {
            assert!(kattis_builddeps().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_oriented());
        }

        #[test]
        fn is_oriented_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_oriented());
        }
    };
}
