//! # Graaf
//!
//! Rust-powered directed graphs.
//!
//! # Table of Contents
//!
//! - [Digraph Types](#digraph-types)
//! - [Creating Digraphs](#creating-digraphs)
//! - [Operations](#operations)
//! - [Algorithms](#algorithms)
//!    - [Bellman-Ford-Moore](#bellman-ford-moore)
//!    - [Breadth-First Search](#breadth-first-search-bfs)
//!    - [Depth-First Search](#depth-first-search-dfs)
//!    - [Dijkstra](#dijkstra)
//!    - [Floyd-Warshall](#floyd-warshall)
//!    - [Tarjan](#tarjan)
//!    - [Breath-First Tree](#breadth-first-tree)
//!    - [Distance Matrix](#distance-matrix)
//!
//! # Digraph Types
//!
//! Graaf provides directed graphs representations. These types eagerly
//! implement digraph [operations](#operations) and [algorithms](#algorithms).
//!
//! - [`adjacency_list`] represents unweighted sparse digraphs.
//! - [`adjacency_matrix`] represents unweighted dense digraphs.
//! - [`adjacency_list_weighted`] represents arc-weighted sparse digraphs.
//! - [`edge_list`] represents unweighted sparse digraphs.
//!
//! # Creating Digraphs
//!
//! The [`gen`] module provides digraph generators. Each digraph representation
//! can be constructed with the operations in the [`op`] module.
//!
//! - [`Biclique`](gen::Biclique) generates a complete bipartite digraph.
//! - [`Circuit`](gen::Circuit) generates a circuit digraph.
//! - [`Complete`](gen::Complete) generates a complete digraph.
//! - [`Cycle`](gen::Cycle) generates a bidirectional circuit.
//! - [`Empty`](gen::Empty) generates a digraph with no arcs.
//! - [`ErdosRenyi`](gen::ErdosRenyi) generates a random digraph.
//! - [`RandomTournament`](gen::RandomTournament) generates a random
//!   tournament.
//! - [`Star`](gen::Star) generates a star digraph.
//! - [`Path`](gen::Path) generates a path digraph.
//!
//! # Operations
//!
//! The [`op`] module provides digraph operation traits. The [digraph
//! types](#digraph-types) implement these traits. One can implement these
//! traits for custom digraph types. Operations form the foundation for
//! [algorithms](#algorithms).
//!
//! - [`AddArcWeighted`](op::AddArcWeighted) adds an arc to an arc-weighted
//!   digraph.
//! - [`AddArc`](op::AddArc) adds an arc to an unweighted digraph.
//! - [`ArcWeight`](op::ArcWeight) returns an arc's weight.
//! - [`ArcsWeighted`](op::ArcsWeighted) returns a digraph's weighted arcs.
//! - [`Arcs`](op::Arcs) returns a digraph's arcs.
//! - [`Complement`](op::Complement) returns a digraph's complement.
//! - [`Converse`](op::Converse) returns a digraph's converse.
//! - [`DegreeSequence`](op::DegreeSequence) returns a digraph's degree
//!   sequence.
//! - [`Degree`](op::Degree) returns a vertex's degree.
//! - [`HasArc`](op::HasArc) checks whether a digraph contains an arc.
//! - [`HasEdge`](op::HasEdge) checks whether a digraph contains an edge.
//! - [`HasWalk`](op::HasWalk) checks whether a digraph contains a walk.
//! - [`InNeighbors`](op::InNeighbors) returns a vertex's in-neighbors.
//! - [`IndegreeSequence`](op::IndegreeSequence) returns a digraph's indegree
//!   sequence.
//! - [`Indegree`](op::Indegree) a vertex's indegree.
//! - [`IsBalanced`](op::IsBalanced) checks whether a digraph is balanced.
//! - [`IsComplete`](op::IsComplete) checks whether a digraph is complete.
//! - [`IsIsolated`](op::IsIsolated) checks whether a vertex is isolated.
//! - [`IsOriented`](op::IsOriented) checks whether a digraph is oriented.
//! - [`IsPendant`](op::IsPendant) checks whether a vertex is a pendant.
//! - [`IsRegular`](op::IsRegular) checks whether a digraph is regular.
//! - [`IsSemicomplete`](op::IsSemicomplete) checks whether a digraph is
//!   semicomplete.
//! - [`IsSimple`](op::IsSimple) checks whether a digraph is simple.
//! - [`IsSpanningSubdigraph`](op::IsSpanningSubdigraph) checks whether a
//!   digraph spans a superdigraph.
//! - [`IsSubdigraph`](op::IsSubdigraph) checks whether a digraph is a
//!   subdigraph.
//! - [`IsSuperdigraph`](op::IsSuperdigraph) checks whether a digraph is a
//!   superdigraph.
//! - [`IsSymmetric`](op::IsSymmetric) checks whether a digraph is symmetric.
//! - [`IsTournament`](op::IsTournament) checks whether a digraph is a
//!   tournament.
//! - [`Order`](op::Order) returns the number of vertices in a digraph.
//! - [`OutNeighborsWeighted`](op::OutNeighborsWeighted) returns a vertex's
//!   weighted out-neighbors.
//! - [`OutNeighbors`](op::OutNeighbors) returns a vertex's out-neighbors.
//! - [`OutdegreeSequence`](op::OutdegreeSequence) returns a digraph's
//!   outdegree sequence.
//! - [`Outdegree`](op::Outdegree) returns a vertex's outdegree.
//! - [`RemoveArc`](op::RemoveArc) removes an arc from a digraph.
//! - [`SemidegreeSequence`](op::SemidegreeSequence) returns a digraph's
//!   semidegree sequence.
//! - [`Sinks`](op::Sinks) returns a digraph's sinks.
//! - [`Size`](op::Size) returns the number of arcs in a digraph.
//! - [`Sources`](op::Sources) returns a digraph's sources.
//! - [`Vertices`](op::Vertices) returns a digraph's vertices.
//!
//! # Algorithms
//!
//! The [`algo`] module provides digraph algorithms.
//!
//! ## Bellman-Ford-Moore
//!
//! The Bellman-Ford-Moore algorithm finds the shortest paths in an
//! arc-weighted digraph with negative weights.
//!
//! - [`single_source_distances`](algo::bellman_ford_moore::single_source_distances) finds the shortest distances.
//!
//! ## Breadth-First Search
//!
//! A breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source.
//!
//! - [`Bfs`](algo::bfs::Bfs) iterates over the vertices.
//! - [`BfsDist`](algo::bfs_dist::BfsDist) iterates over the vertices and their
//!   distance from the source.
//! - [`BfsPred`](algo::bfs_pred::BfsPred) iterates over the vertices and their
//!   predecessors.
//! - [`BfsDist::distances`](algo::bfs_dist::BfsDist::distances) finds the
//!   shortest distances.
//! - [`BfsPred::predecessors`](algo::bfs_pred::BfsPred::predecessors) finds
//!   the predecessors.
//! - [`BfsPred::shortest_path`](algo::bfs_pred::BfsPred::shortest_path) finds
//!   the shortest path.

//! ## Depth-First Search
//!
//! A depth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source.
//!
//! - [`Dfs`](algo::dfs::Dfs) iterates over the vertices.
//! - [`DfsDist`](algo::dfs_dist::DfsDist) iterates over the vertices and their
//!   distance from the source.
//! - [`DfsPred`](algo::dfs_pred::DfsPred) iterates over the vertices and their
//!   predecessors.
//!
//! ## Dijkstra
//!
//! Dijkstra's algorithm finds the shortest paths from one or more source
//! vertices in an arc-weighted digraph.
//!
//! - [`Dijkstra`](algo::dijkstra::Dijkstra) iterates over the vertices.
//! - [`DijkstraDist`](algo::dijkstra_dist::DijkstraDist) iterates over the
//!   vertices and their distance from the source.
//! - [`DijkstraPred`](algo::dijkstra_pred::DijkstraPred) iterates over the
//!   vertices and their predecessors.
//! - [`DijkstraDist::distances`](algo::dijkstra_dist::DijkstraDist::distances)
//!   finds the shortest distances.
//! - [`DijkstraPred::predecessors`](algo::dijkstra_pred::DijkstraPred::predecessors) finds the predecessors.
//! - [`DijkstraPred::shortest_path`](algo::dijkstra_pred::DijkstraPred::shortest_path) finds the shortest path.
//!
//! ## Floyd-Warshall
//!
//! The Floyd-Warshall algorithm finds the shortest paths between all pairs
//! of vertices in an arc-weighted digraph.
//!
//! - [`distances`](algo::floyd_warshall::distances) finds the shortest
//!   distances.
//!
//! ## Tarjan
//!
//! Tarjan's algorithm finds the strongly connected components in a digraph.
//!
//! - [`strongly_connected_components`](algo::tarjan::strongly_connected_components) finds the strongly connected components.
//!
//! ## Predecessor Tree
//!
//! A [`PredecessorTree`](algo::PredecessorTree) is the result of a
//! breadth-first search and contains the predecessors of the vertices on the
//! shortest paths.
//!
//! - [`PredecessorTree::search`](algo::PredecessorTree::search) finds a vertex
//!   by value.
//! - [`PredecessorTree::search_by`](algo::PredecessorTree::search_by) finds a
//!   vertex by predicate.
//!
//! These functions produce a predecessor tree.
//!
//! - [`BfsPred::predecessors`](algo::bfs_pred::BfsPred::predecessors)
//! - [`DijkstraPred::predecessors`](algo::dijkstra_pred::DijkstraPred::predecessors)
//!
//! ## Distance Matrix
//!
//! A [`DistanceMatrix`](algo::DistanceMatrix) contains the shortest distances
//! between all pairs of vertices in a digraph.
//!
//! - [`DistanceMatrix::center`](algo::DistanceMatrix::center) finds the center
//!   of the digraph.
//! - [`DistanceMatrix::diameter`](algo::DistanceMatrix::diameter) finds the
//!   diameter of the digraph.
//! - [`DistanceMatrix::eccentricities`](algo::DistanceMatrix::eccentricities)
//!   returns the eccentricities of the vertices.
//! - [`DistanceMatrix::is_connected`](algo::DistanceMatrix::is_connected)
//!   checks if the digraph is connected.
//! - [`DistanceMatrix::periphery`](algo::DistanceMatrix::periphery) finds the
//!   periphery of the digraph.

// Clippy lint groups
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
// Clippy restriction lints
#![deny(
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::missing_assert_message,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::self_named_module_files,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result
)]
// Rustc lint groups
#![deny(rust_2018_idioms)]
// Rustc lints
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]
// Rustdoc lints
#![deny(rustdoc::all)]

pub mod adjacency_list;
pub mod adjacency_list_weighted;
pub mod adjacency_matrix;
pub mod algo;
pub mod edge_list;
pub mod gen;
pub mod op;
#[cfg(test)]
pub mod proptest_strategy;
