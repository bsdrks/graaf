//! Check whether a sequence is a walk in a digraph.
//!
//! A sequence of vertices is a walk in a digraph if each pair of consecutive
//! vertices in the sequence is an arc in the digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     HasWalk,
//! };
//!
//! let digraph = AdjacencyList::circuit(2);
//!
//! assert!(digraph.has_walk(&[0, 1]));
//! assert!(digraph.has_walk(&[1, 0]));
//!
//! assert!(!digraph.has_walk(&[0]));
//! assert!(!digraph.has_walk(&[1]));
//! assert!(!digraph.has_walk(&[2]));
//! assert!(!digraph.has_walk(&[0, 0]));
//! assert!(!digraph.has_walk(&[1, 1]));
//! assert!(!digraph.has_walk(&[0, 2]));
//! ```

/// Digraph walk
pub trait HasWalk {
    /// Check whether the sequence is a walk in the digraph.
    ///
    /// # Arguments
    ///
    /// * `walk`: A sequence of vertices.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     HasWalk,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(2);
    ///
    /// assert!(digraph.has_walk(&[0, 1]));
    /// assert!(digraph.has_walk(&[1, 0]));
    ///
    /// assert!(!digraph.has_walk(&[0]));
    /// assert!(!digraph.has_walk(&[1]));
    /// assert!(!digraph.has_walk(&[2]));
    /// assert!(!digraph.has_walk(&[0, 0]));
    /// assert!(!digraph.has_walk(&[1, 1]));
    /// assert!(!digraph.has_walk(&[0, 2]));
    /// ```
    #[must_use]
    fn has_walk(&self, walk: &[usize]) -> bool;
}

/// `HasWalk` tests
#[macro_export]
macro_rules! test_has_walk {
    ($fixture:path) => {
        use $fixture::{
            bang_jensen_34,
            bang_jensen_94,
            kattis_builddeps,
            kattis_escapewallmaria_1,
            kattis_escapewallmaria_2,
            kattis_escapewallmaria_3,
        };

        #[test]
        fn has_walk_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.has_walk(&[0, 4]));
            assert!(digraph.has_walk(&[1, 0, 4]));
            assert!(digraph.has_walk(&[2, 1, 0, 4]));
            assert!(digraph.has_walk(&[2, 3]));
            assert!(digraph.has_walk(&[2, 5, 4]));
            assert!(digraph.has_walk(&[5, 4]));

            assert!(!digraph.has_walk(&[0, 1]));
            assert!(!digraph.has_walk(&[1, 2]));
            assert!(!digraph.has_walk(&[2, 0]));
            assert!(!digraph.has_walk(&[3]));
            assert!(!digraph.has_walk(&[4]));
            assert!(!digraph.has_walk(&[5, 0]));
        }

        #[test]
        fn has_walk_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.has_walk(&[0, 1, 3, 5]));
            assert!(digraph.has_walk(&[0, 2, 1, 3, 5]));
            assert!(digraph.has_walk(&[0, 2, 3, 5]));
            assert!(digraph.has_walk(&[0, 2, 4, 6]));
            assert!(digraph.has_walk(&[0, 2, 5]));
            assert!(digraph.has_walk(&[1, 3, 5]));
            assert!(digraph.has_walk(&[2, 1, 3, 5]));
            assert!(digraph.has_walk(&[2, 3, 5]));
            assert!(digraph.has_walk(&[2, 4, 6]));
            assert!(digraph.has_walk(&[2, 5]));
            assert!(digraph.has_walk(&[3, 5]));
            assert!(digraph.has_walk(&[4, 6]));

            assert!(!digraph.has_walk(&[0, 3]));
            assert!(!digraph.has_walk(&[1, 0]));
            assert!(!digraph.has_walk(&[2, 0]));
            assert!(!digraph.has_walk(&[3, 0]));
            assert!(!digraph.has_walk(&[4, 0]));
            assert!(!digraph.has_walk(&[5]));
            assert!(!digraph.has_walk(&[6]));
        }

        #[test]
        fn has_walk_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.has_walk(&[0, 3, 1]));
            assert!(digraph.has_walk(&[0, 4, 1]));
            assert!(digraph.has_walk(&[2, 3, 1]));
            assert!(digraph.has_walk(&[2, 4, 1]));
            assert!(digraph.has_walk(&[2, 5, 1]));
            assert!(digraph.has_walk(&[3, 1]));
            assert!(digraph.has_walk(&[4, 1]));
            assert!(digraph.has_walk(&[5, 1]));

            assert!(!digraph.has_walk(&[0, 1]));
            assert!(!digraph.has_walk(&[1]));
            assert!(!digraph.has_walk(&[2, 0]));
            assert!(!digraph.has_walk(&[3, 0]));
            assert!(!digraph.has_walk(&[4, 0]));
            assert!(!digraph.has_walk(&[5, 0]));
        }

        #[test]
        fn has_walk_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.has_walk(&[5, 6, 5]));
            assert!(digraph.has_walk(&[5, 9, 5]));
            assert!(digraph.has_walk(&[5, 9, 13, 9]));
            assert!(digraph.has_walk(&[5, 9, 13, 12]));
            assert!(digraph.has_walk(&[6, 5, 6]));
            assert!(digraph.has_walk(&[6, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[6, 5, 9, 13, 12]));
            assert!(digraph.has_walk(&[9, 5, 6, 5]));
            assert!(digraph.has_walk(&[9, 5, 9]));
            assert!(digraph.has_walk(&[9, 13, 9]));
            assert!(digraph.has_walk(&[9, 13, 12]));
            assert!(digraph.has_walk(&[13, 9, 5, 6, 5]));
            assert!(digraph.has_walk(&[13, 9, 5, 9]));
            assert!(digraph.has_walk(&[13, 9, 13]));
            assert!(digraph.has_walk(&[13, 12]));

            assert!(!digraph.has_walk(&[0]));
            assert!(!digraph.has_walk(&[1]));
            assert!(!digraph.has_walk(&[2]));
            assert!(!digraph.has_walk(&[3]));
            assert!(!digraph.has_walk(&[4]));
            assert!(!digraph.has_walk(&[5, 0]));
            assert!(!digraph.has_walk(&[6, 0]));
            assert!(!digraph.has_walk(&[7]));
            assert!(!digraph.has_walk(&[8]));
            assert!(!digraph.has_walk(&[9, 0]));
            assert!(!digraph.has_walk(&[10]));
            assert!(!digraph.has_walk(&[11]));
            assert!(!digraph.has_walk(&[12]));
            assert!(!digraph.has_walk(&[13, 0]));
        }

        #[test]
        fn has_walk_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.has_walk(&[5, 6, 5]));
            assert!(digraph.has_walk(&[5, 9, 5]));
            assert!(digraph.has_walk(&[6, 5, 6]));
            assert!(digraph.has_walk(&[6, 5, 9, 5]));
            assert!(digraph.has_walk(&[9, 5, 6, 5]));
            assert!(digraph.has_walk(&[9, 5, 9]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 5]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 9]));
            assert!(digraph.has_walk(&[12, 13, 12]));
            assert!(digraph.has_walk(&[13, 9, 5, 6, 5]));
            assert!(digraph.has_walk(&[13, 9, 5, 9]));
            assert!(digraph.has_walk(&[13, 12, 13]));

            assert!(!digraph.has_walk(&[0]));
            assert!(!digraph.has_walk(&[1]));
            assert!(!digraph.has_walk(&[2]));
            assert!(!digraph.has_walk(&[3]));
            assert!(!digraph.has_walk(&[4]));
            assert!(!digraph.has_walk(&[5, 0]));
            assert!(!digraph.has_walk(&[6, 0]));
            assert!(!digraph.has_walk(&[7]));
            assert!(!digraph.has_walk(&[8]));
            assert!(!digraph.has_walk(&[9, 0]));
            assert!(!digraph.has_walk(&[10]));
            assert!(!digraph.has_walk(&[11]));
            assert!(!digraph.has_walk(&[12, 0]));
            assert!(!digraph.has_walk(&[13, 0]));
        }

        #[test]
        fn has_walk_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.has_walk(&[1, 2, 1]));
            assert!(digraph.has_walk(&[1, 2, 6, 2]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 1]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 6]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 9, 5]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 9, 13]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[1, 2, 6, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[1, 5, 1]));
            assert!(digraph.has_walk(&[1, 5, 6, 2, 1]));
            assert!(digraph.has_walk(&[1, 5, 6, 2, 6]));
            assert!(digraph.has_walk(&[1, 5, 9, 5]));
            assert!(digraph.has_walk(&[1, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[1, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[2, 1, 2]));
            assert!(digraph.has_walk(&[2, 1, 5, 1]));
            assert!(digraph.has_walk(&[2, 1, 5, 6, 2]));
            assert!(digraph.has_walk(&[2, 1, 5, 6, 5]));
            assert!(digraph.has_walk(&[2, 1, 5, 9, 5]));
            assert!(digraph.has_walk(&[2, 1, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[2, 1, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[2, 6, 2]));
            assert!(digraph.has_walk(&[2, 6, 5, 1, 2]));
            assert!(digraph.has_walk(&[2, 6, 5, 1, 5]));
            assert!(digraph.has_walk(&[2, 6, 5, 6]));
            assert!(digraph.has_walk(&[2, 6, 5, 9, 5]));
            assert!(digraph.has_walk(&[2, 6, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[2, 6, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[5, 1, 2, 1]));
            assert!(digraph.has_walk(&[5, 1, 2, 6, 2]));
            assert!(digraph.has_walk(&[5, 1, 2, 6, 5]));
            assert!(digraph.has_walk(&[5, 1, 5]));
            assert!(digraph.has_walk(&[5, 6, 2, 1, 2]));
            assert!(digraph.has_walk(&[5, 6, 2, 1, 5]));
            assert!(digraph.has_walk(&[5, 6, 2, 6]));
            assert!(digraph.has_walk(&[5, 6, 5]));
            assert!(digraph.has_walk(&[5, 9, 5]));
            assert!(digraph.has_walk(&[5, 9, 13, 9]));
            assert!(digraph.has_walk(&[5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[6, 2, 1, 2]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 1]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 6]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 9, 5]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[6, 2, 1, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[6, 2, 6]));
            assert!(digraph.has_walk(&[6, 5, 1, 2, 1]));
            assert!(digraph.has_walk(&[6, 5, 1, 5]));
            assert!(digraph.has_walk(&[6, 5, 6]));
            assert!(digraph.has_walk(&[6, 5, 9, 5]));
            assert!(digraph.has_walk(&[6, 5, 9, 13, 9]));
            assert!(digraph.has_walk(&[6, 5, 9, 13, 12, 13]));
            assert!(digraph.has_walk(&[9, 5, 1, 2, 1]));
            assert!(digraph.has_walk(&[9, 5, 1, 2, 6, 2]));
            assert!(digraph.has_walk(&[9, 5, 1, 2, 6, 5]));
            assert!(digraph.has_walk(&[9, 5, 1, 5]));
            assert!(digraph.has_walk(&[9, 5, 6, 2, 1, 2]));
            assert!(digraph.has_walk(&[9, 5, 6, 2, 1, 5]));
            assert!(digraph.has_walk(&[9, 5, 6, 2, 6]));
            assert!(digraph.has_walk(&[9, 5, 6, 5]));
            assert!(digraph.has_walk(&[9, 5, 9]));
            assert!(digraph.has_walk(&[9, 13, 9]));
            assert!(digraph.has_walk(&[9, 13, 12, 13]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 1, 2, 1]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 1, 5]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 2, 1, 2]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 2, 1, 5]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 2, 6]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 6, 5]));
            assert!(digraph.has_walk(&[12, 13, 9, 5, 9]));
            assert!(digraph.has_walk(&[12, 13, 9, 13]));
            assert!(digraph.has_walk(&[12, 13, 12]));

            assert!(!digraph.has_walk(&[0]));
            assert!(!digraph.has_walk(&[1, 0]));
            assert!(!digraph.has_walk(&[2, 0]));
            assert!(!digraph.has_walk(&[3]));
            assert!(!digraph.has_walk(&[4]));
            assert!(!digraph.has_walk(&[5, 0]));
            assert!(!digraph.has_walk(&[6, 0]));
            assert!(!digraph.has_walk(&[7]));
            assert!(!digraph.has_walk(&[8]));
            assert!(!digraph.has_walk(&[9, 0]));
            assert!(!digraph.has_walk(&[10]));
            assert!(!digraph.has_walk(&[11]));
            assert!(!digraph.has_walk(&[12, 0]));
            assert!(!digraph.has_walk(&[13, 0]));
        }
    };
}
