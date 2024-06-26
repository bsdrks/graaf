//! Determine whether a digraph is semicomplete.
//!
//! A digraph is semicomplete if, for every pair `s`, `t` of distinct vertices,
//! there is an arc between `s` and `t`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::{
//!         Complete,
//!         Cycle,
//!         Empty,
//!         RandomTournament,
//!     },
//!     op::IsSemicomplete,
//! };
//!
//! assert!(!Digraph::empty(3).is_semicomplete());
//! assert!(Digraph::complete(3).is_semicomplete());
//! assert!(Digraph::cycle(3).is_semicomplete());
//! assert!(Digraph::random_tournament(3).is_semicomplete());
//! ```

use crate::op::{
    HasArc,
    Order,
};

/// Determine whether a digraph is semicomplete.
///
/// # How can I implement `IsSemicomplete`?
///
/// Provide an implementation of `is_semicomplete` that returns `true` if there
/// is an arc between every pair `s`, `t` of distinct vertices OR implement
/// `HasArc` and `Order`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Complete,
///             Cycle,
///             Empty,
///         },
///         op::{
///             HasArc,
///             IsSemicomplete,
///             Order,
///         },
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, s: usize, t: usize) -> bool {
///         self.arcs[s].contains(&t)
///     }
/// }
///
/// impl Order for Digraph {
///     fn order(&self) -> usize {
///         self.arcs.len()
///     }
/// }
///
/// assert!(Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0, 1]),
///     ]
/// }
/// .is_semicomplete());
///
/// assert!(Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0]),
///     ]
/// }
/// .is_semicomplete());
///
/// assert!(!Digraph {
///     arcs: vec![BTreeSet::new(); 3]
/// }
/// .is_semicomplete());
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::{
///         Complete,
///         Cycle,
///         Empty,
///         RandomTournament,
///     },
///     op::IsSemicomplete,
/// };
///
/// assert!(!Digraph::empty(3).is_semicomplete());
/// assert!(Digraph::complete(3).is_semicomplete());
/// assert!(Digraph::cycle(3).is_semicomplete());
/// assert!(Digraph::random_tournament(3).is_semicomplete());
/// ```
pub trait IsSemicomplete {
    /// Determines whether the digraph is semicomplete.
    fn is_semicomplete(&self) -> bool;
}

impl<D> IsSemicomplete for D
where
    D: HasArc + Order,
{
    fn is_semicomplete(&self) -> bool {
        let v = self.order();

        for s in 0..v {
            for t in (s + 1)..v {
                if !(self.has_arc(s, t) || self.has_arc(t, s)) {
                    return false;
                }
            }
        }

        true
    }
}
