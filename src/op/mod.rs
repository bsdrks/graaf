//! Operations on graphs
//!
//! These operations are modeled as traits that can be implemented by types
//! that represent graphs. The traits are implemented for combinations of
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
//!         AddEdge,
//!         Indegree,
//!         Outdegree,
//!         RemoveEdge,
//!     },
//! };
//!
//! let mut graph = vec![BTreeSet::new(); 3];
//!
//! // 1 ← 0 → 2
//!
//! graph.add_edge(0, 1);
//! graph.add_edge(0, 2);
//!
//! assert_eq!(graph.outdegree(0), 2);
//! assert_eq!(graph.indegree(1), 1);
//! assert_eq!(graph.indegree(2), 1);
//!
//! graph.remove_edge(0, 1);
//!
//! assert_eq!(graph.outdegree(0), 1);
//! assert_eq!(graph.indegree(1), 0);
//! assert_eq!(graph.indegree(2), 1);
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
//! ## Edge list
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

pub mod add_edge;
pub mod add_weighted_edge;
pub mod count_all_edges;
pub mod count_all_vertices;
pub mod degree;
pub mod edge_weight;
pub mod has_edge;
pub mod has_edge_symmetric;
pub mod indegree;
pub mod is_balanced;
pub mod is_isolated;
pub mod is_pendant;
pub mod is_regular;
pub mod is_simple;
pub mod is_symmetric;
pub mod iter_all_edges;
pub mod iter_all_weighted_edges;
pub mod iter_edges;
pub mod iter_vertices;
pub mod iter_weighted_edges;
pub mod outdegree;
pub mod remove_edge;

pub use {
    add_edge::AddEdge,
    add_weighted_edge::AddWeightedEdge,
    count_all_edges::CountAllEdges,
    count_all_vertices::CountAllVertices,
    degree::Degree,
    edge_weight::EdgeWeight,
    has_edge::HasEdge,
    has_edge_symmetric::HasEdgeSymmetric,
    indegree::Indegree,
    is_balanced::IsBalanced,
    is_isolated::IsIsolated,
    is_pendant::IsPendant,
    is_regular::IsRegular,
    is_simple::IsSimple,
    is_symmetric::IsSymmetric,
    iter_all_edges::IterAllEdges,
    iter_all_weighted_edges::IterAllWeightedEdges,
    iter_edges::IterEdges,
    iter_vertices::IterVertices,
    iter_weighted_edges::IterWeightedEdges,
    outdegree::Outdegree,
    remove_edge::RemoveEdge,
};
