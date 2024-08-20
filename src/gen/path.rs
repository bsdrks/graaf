//! Generate path digraphs.
//!
//! A path digraph is a chain of arcs that connect vertices in a linear
//! sequence.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a path digraph of order 2.
//!
//! ![Path digraph of order 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_2.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Path,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::path(2).arcs().eq([(0, 1)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a path digraph of order 3.
//!
//! ![Path digraph of order 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_3.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Path,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::path(3).arcs().eq([(0, 1), (1, 2)]));
//! ```
//!
//! ## Order 4
//!
//! Generate a path digraph of order 4.
//!
//! ![Path digraph of order 4](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Path,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
//! ```

use crate::{
    gen::Empty,
    op::AddArc,
};

/// Generate path digraphs.
///
/// # Implementing `Path`
///
/// Provide an implementation of `path` that generates a path digraph of a
/// given `order` OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Empty,
///             Path,
///         },
///         op::AddArc,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl AddArc for Digraph {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// impl Empty for Digraph {
///     fn empty(order: usize) -> Self {
///         Self {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {}
///
/// let digraph = Digraph::path(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1]),
///     BTreeSet::from([2]),
///     BTreeSet::new()
/// ]));
/// ```
pub trait Path {
    /// Generates a path digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a path digraph of order 2.
    ///
    /// ![Path digraph of order 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Path,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::path(2).arcs().eq([(0, 1)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a path digraph of order 3.
    ///
    /// ![Path digraph of order 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Path,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::path(3).arcs().eq([(0, 1), (1, 2)]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a path digraph of order 4.
    ///
    /// ![Path digraph of order 4](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Path,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
    /// ```
    #[must_use]
    fn path(order: usize) -> Self;
}

impl<D> Path for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn path(order: usize) -> Self {
        let mut digraph = D::empty(order);

        if order == 1 {
            return digraph;
        }

        for u in 0..order - 1 {
            let v = u + 1;

            digraph.add_arc(u, v);
        }

        digraph
    }
}
