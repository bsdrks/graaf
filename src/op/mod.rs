//! Operations on digraphs
//!
//! These operations are modeled as traits that can be implemented by types
//! that represent digraphs. The traits are implemented for combinations of
//! standard types like [`array`], [`slice`], [`Vec`], [`BTreeMap`],
//! [`BTreeSet`], [`HashMap`], and [`HashSet`] when the implementation has a
//! close-to-optimal complexity.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::op::{
//!         AddArc,
//!         Indegree,
//!         Outdegree,
//!         RemoveArc,
//!     },
//! };
//!
//! let mut digraph = vec![BTreeSet::new(); 3];
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//!
//! assert_eq!(digraph.outdegree(0), 2);
//! assert_eq!(digraph.indegree(1), 1);
//! assert_eq!(digraph.indegree(2), 1);
//!
//! digraph.remove_arc(0, 1);
//!
//! assert_eq!(digraph.outdegree(0), 1);
//! assert_eq!(digraph.indegree(1), 0);
//! assert_eq!(digraph.indegree(2), 1);
//! ```
//!
//! # Supported types
//!
//! ## Adjacency list
//!
//! ### Unweighted
//!
//! - `BTreeMap<usize, BTreeSet<usize>>`
//! - `BTreeMap<usize, Vec<usize>>`
//! - `HashMap<usize, HashSet<usize>>`
//! - `HashMap<usize, Vec<usize>>`
//! - `Vec<BTreeSet<usize>>`
//! - `Vec<HashSet<usize>>`
//! - `Vec<Vec<usize>>`
//! - `[BTreeSet<usize>; V]`
//! - `[BTreeSet<usize>]`
//! - `[HashSet<usize>; V]`
//! - `[HashSet<usize>]`
//! - `[Vec<usize>; V]`
//! - `[Vec<usize>]`
//!
//! ### Weighted
//!
//! - `BTreeMap<usize, BTreeMap<usize, W>>`
//! - `BTreeMap<usize, BTreeSet<(usize, W)>>`
//! - `BTreeMap<usize, Vec<(usize, W)>>`
//! - `HashMap<usize, HashMap<usize, W>>`
//! - `HashMap<usize, HashSet<(usize, W)>>`
//! - `HashMap<usize, Vec<(usize, W)>>`
//! - `Vec<BTreeMap<usize, W>>`
//! - `Vec<BTreeSet<(usize, W)>>`
//! - `Vec<HashMap<usize, W>>`
//! - `Vec<HashSet<(usize, W)>>`
//! - `Vec<Vec<(usize, W)>>`
//! - `[BTreeMap<usize, W>; V]`
//! - `[BTreeMap<usize, W>]`
//! - `[BTreeSet<(usize, W)>; V]`
//! - `[BTreeSet<(usize, W)>]`
//! - `[HashMap<usize, W>; V]`
//! - `[HashMap<usize, W>]`
//! - `[HashSet<(usize, W)>; V]`
//! - `[HashSet<(usize, W)>]`
//! - `[Vec<(usize, W)>; V]`
//! - `[Vec<(usize, W)>]`
//!
//! ## Arc list
//!
//! ### Unweighted
//!
//! - `BTreeSet<(usize, usize)>`
//! - `HashSet<(usize, usize)>`
//! - `Vec<(usize, usize)>`
//! - `[(usize, usize); V]`
//! - `[(usize, usize)]`
//!
//! ### Weighted
//!
//! - `BTreeSet<(usize, usize, W)>`
//! - `HashSet<(usize, usize, W)>`
//! - `Vec<(usize, usize, W)>`
//! - `[(usize, usize, W); V]`
//! - `[(usize, usize, W)]`
//!
//! [`BTreeMap`]: std::collections::BTreeMap
//! [`BTreeSet`]: std::collections::BTreeSet
//! [`HashMap`]: std::collections::HashMap
//! [`HashSet`]: std::collections::HashSet
//! [`Vec`]: std::vec::Vec
//! [`array`]: https://doc.rust-lang.org/std/primitive.array.html
//! [`slice`]: https://doc.rust-lang.org/std/primitive.slice.html

pub mod add_arc;
pub mod add_weighted_arc;
pub mod arc_weight;
pub mod converse;
pub mod degree;
pub mod has_arc;
pub mod has_edge;
pub mod indegree;
pub mod is_balanced;
pub mod is_isolated;
pub mod is_oriented;
pub mod is_pendant;
pub mod is_regular;
pub mod is_simple;
pub mod is_subdigraph;
pub mod is_symmetric;
pub mod is_walk;
pub mod iter_arcs;
pub mod iter_out_neighbors;
pub mod iter_out_neighbors_mut;
pub mod iter_out_weighted_neighbors;
pub mod iter_vertices;
pub mod iter_weighted_arcs;
pub mod order;
pub mod outdegree;
pub mod remove_arc;
pub mod reverse_arc;
pub mod size;

pub use {
    add_arc::AddArc,
    add_weighted_arc::AddWeightedArc,
    arc_weight::ArcWeight,
    converse::Converse,
    degree::Degree,
    has_arc::HasArc,
    has_edge::HasEdge,
    indegree::Indegree,
    is_balanced::IsBalanced,
    is_isolated::IsIsolated,
    is_oriented::IsOriented,
    is_pendant::IsPendant,
    is_regular::IsRegular,
    is_simple::IsSimple,
    is_subdigraph::IsSubdigraph,
    is_symmetric::IsSymmetric,
    is_walk::IsWalk,
    iter_arcs::IterArcs,
    iter_out_neighbors::IterOutNeighbors,
    iter_out_neighbors_mut::IterOutNeighborsMut,
    iter_out_weighted_neighbors::IterOutWeightedNeighbors,
    iter_vertices::IterVertices,
    iter_weighted_arcs::IterWeightedArcs,
    order::Order,
    outdegree::Outdegree,
    remove_arc::RemoveArc,
    reverse_arc::ReverseArc,
    size::Size,
};
