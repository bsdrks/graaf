//! Operations on digraphs
//!
//! These traits are digraph operations that can be implemented by digraph
//! representations.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IndegreeSequence,
//!     OutdegreeSequence,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//!
//! assert!(digraph.indegree_sequence().eq([0, 1, 1]));
//! assert!(digraph.outdegree_sequence().eq([2, 0, 0]));
//!
//! assert!(digraph.remove_arc(0, 1));
//!
//! assert!(digraph.indegree_sequence().eq([0, 0, 1]));
//! assert!(digraph.outdegree_sequence().eq([1, 0, 0]));
//! ```

pub mod add_arc;
pub mod add_arc_weighted;
pub mod arc_weight;
pub mod arcs;
pub mod arcs_weighted;
pub mod complement;
pub mod converse;
pub mod degree;
pub mod degree_sequence;
pub mod has_arc;
pub mod has_edge;
pub mod has_walk;
pub mod in_neighbors;
pub mod indegree;
pub mod indegree_sequence;
pub mod is_balanced;
pub mod is_complete;
pub mod is_isolated;
pub mod is_oriented;
pub mod is_pendant;
pub mod is_regular;
pub mod is_semicomplete;
pub mod is_simple;
pub mod is_spanning_subdigraph;
pub mod is_subdigraph;
pub mod is_superdigraph;
pub mod is_symmetric;
pub mod is_tournament;
pub mod order;
pub mod out_neighbors;
pub mod out_neighbors_weighted;
pub mod outdegree;
pub mod outdegree_sequence;
pub mod remove_arc;
pub mod semidegree_sequence;
pub mod sinks;
pub mod size;
pub mod sources;
pub mod union;
pub mod vertices;

pub use {
    add_arc::AddArc,
    add_arc_weighted::AddArcWeighted,
    arc_weight::ArcWeight,
    arcs::Arcs,
    arcs_weighted::ArcsWeighted,
    complement::Complement,
    converse::Converse,
    degree::Degree,
    degree_sequence::DegreeSequence,
    has_arc::HasArc,
    has_edge::HasEdge,
    has_walk::HasWalk,
    in_neighbors::InNeighbors,
    indegree::Indegree,
    indegree_sequence::IndegreeSequence,
    is_balanced::IsBalanced,
    is_complete::IsComplete,
    is_isolated::IsIsolated,
    is_oriented::IsOriented,
    is_pendant::IsPendant,
    is_regular::IsRegular,
    is_semicomplete::IsSemicomplete,
    is_simple::IsSimple,
    is_spanning_subdigraph::IsSpanningSubdigraph,
    is_subdigraph::IsSubdigraph,
    is_superdigraph::IsSuperdigraph,
    is_symmetric::IsSymmetric,
    is_tournament::IsTournament,
    order::Order,
    out_neighbors::OutNeighbors,
    out_neighbors_weighted::OutNeighborsWeighted,
    outdegree::Outdegree,
    outdegree_sequence::OutdegreeSequence,
    remove_arc::RemoveArc,
    semidegree_sequence::SemidegreeSequence,
    sinks::Sinks,
    size::Size,
    sources::Sources,
    union::Union,
    vertices::Vertices,
};
