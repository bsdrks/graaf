//! Generate empty digraphs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         Arcs,
//!         Order,
//!     },
//! };
//!
//! // 0 -> {}
//!
//! assert!(Digraph::empty(1).arcs().eq([]));
//! assert_eq!(Digraph::empty(1).order(), 1);
//!
//! // 0 -> {}
//! // 1 -> {}
//!
//! assert!(Digraph::empty(2).arcs().eq([]));
//! assert_eq!(Digraph::empty(2).order(), 2);
//!
//! // 0 -> {}
//! // 1 -> {}
//! // 2 -> {}
//!
//! assert!(Digraph::empty(3).arcs().eq([]));
//! assert_eq!(Digraph::empty(3).order(), 3);
//! ```
#![doc(alias = "edgeless")]

/// Generate empty digraphs.
///
/// # How can I implement `Empty`?
///
/// Provide an implementation of `empty` that generates an empty digraph with
/// `v` vertices.
///
/// ```
/// use {
///     graaf::gen::Empty,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Empty for Digraph {
///     /// # Panics
///     ///
///     /// Panics if `order` is zero.
///     fn empty(order: usize) -> Self {
///         assert!(order > 0, "a digraph must have at least one vertex");
///
///         Digraph {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// // 0 -> {}
/// // 1 -> {}
/// // 2 -> {}
///
/// assert!(Digraph::empty(3)
///     .arcs
///     .iter()
///     .eq(&[BTreeSet::new(), BTreeSet::new(), BTreeSet::new()]));
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::{
///         adjacency_list::Digraph,
///         gen::Empty,
///         op::{
///             Arcs,
///             Order,
///         },
///     },
///     std::collections::BTreeSet,
/// };
///
/// // 0 -> {}
///
/// assert!(Digraph::empty(1).arcs().eq([]));
/// assert_eq!(Digraph::empty(1).order(), 1);
///
/// // 0 -> {}
/// // 1 -> {}
///
/// assert!(Digraph::empty(2).arcs().eq([]));
/// assert_eq!(Digraph::empty(2).order(), 2);
///
/// // 0 -> {}
/// // 1 -> {}
/// // 2 -> {}
///
/// assert!(Digraph::empty(3).arcs().eq([]));
/// assert_eq!(Digraph::empty(3).order(), 3);
/// ```
#[doc(alias = "Edgeless")]
pub trait Empty {
    /// Generates an empty digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    #[doc(alias = "edgeless")]
    #[must_use]
    fn empty(order: usize) -> Self;

    /// Generates a trivial digraph.
    ///
    /// A trivial digraph has one vertex and no arcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Empty,
    ///     op::Arcs,
    /// };
    ///
    /// // 0 -> {}
    ///
    /// assert!(Digraph::trivial().arcs().eq([]));
    /// ```
    #[doc(alias = "singleton")]
    #[must_use]
    fn trivial() -> Self
    where
        Self: Sized,
    {
        Self::empty(1)
    }
}
