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
//!    - [Breadth-First Search (BFS)](#breadth-first-search-bfs)
//!    - [Depth-First Search (DFS)](#depth-first-search-dfs)
//!    - [Dijkstra](#dijkstra)
//!    - [Floyd-Warshall](#floyd-warshall)
//!    - [Tarjan](#tarjan)
//!    - [Breath-First Tree](#breadth-first-tree)
//!    - [Distance Matrix](#distance-matrix)
//! - [Naming Conventions](#naming-conventions)
//! - [Project Goals](#project-goals)
//!
//! # Digraph Types
//!
//! Graaf provides directed graphs representations.
//!
//! - [`adjacency_list`] represents unweighted sparse digraphs.
//! - [`adjacency_matrix`] represents unweighted dense digraphs.
//! - [`adjacency_list_weighted`] represents arc-weighted sparse digraphs.
//!
//! These types eagerly implement [digraph operations](#operations) and
//! [digraph algorithms](#algorithms).
//!
//! # Creating Digraphs
//!
//! The [`gen`] module provides digraph generators.
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
//! ## Breadth-First Search (BFS)
//!
//! A breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source.
//!
//! - [`bfs::Bfs`](algo::bfs::Bfs) iterates over the vertices.
//! - [`bfs_depth::Bfs::distances`](algo::bfs_depth::Bfs::distances) finds the
//!   shortest distances.
//! - [`bfs_depth::Bfs`](algo::bfs_depth::Bfs) iterates over the vertices and
//!   their depths.
//! - [`bfs_pred::Bfs::predecessors`](algo::bfs_pred::Bfs::predecessors) finds
//!   the predecessors.
//! - [`bfs_pred::Bfs::shortest_path`](algo::bfs_pred::Bfs::shortest_path)
//!   finds the shortest path.
//! - [`bfs_pred::Bfs`](algo::bfs_pred::Bfs) iterates over the vertices and
//!   their predecessors.

//! ## Depth-First Search (DFS)
//!
//! A depth-first search explores the vertices of an unweighted digraph in
//! order of their depth from a source.
//!
//! - [`dfs::Dfs`](algo::dfs::Dfs) iterates over the vertices.
//! - [`dfs_depth::Dfs`](algo::dfs_depth::Dfs) iterates over the vertices and
//!   their depths.
//!
//! ## Dijkstra
//!
//! Dijkstra's algorithm finds the shortest paths from one or more source
//! vertices in an arc-weighted digraph.
//!
//! - [`dijkstra::Dijkstra`](algo::dijkstra::Dijkstra) iterates over the
//!   vertices.
//! - [`dijkstra_dist::Dijkstra::distances`](algo::dijkstra_dist::Dijkstra::distances) finds the shortest distances.
//! - [`dijkstra_dist::Dijkstra`](algo::dijkstra_dist::Dijkstra) iterates over
//!   the vertices and their distances.
//! - [`dijkstra_pred::Dijkstra::predecessors`](algo::dijkstra_pred::Dijkstra::predecessors) finds the predecessors.
//! - [`dijkstra_pred::Dijkstra::shortest_path`](algo::dijkstra_pred::Dijkstra::shortest_path) finds the shortest path.
//! - [`dijkstra_pred::Dijkstra`](algo::dijkstra_pred::Dijkstra) iterates over
//!   the vertices and their predecessors.
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
//! - [`bfs_pred::Bfs::predecessors`](algo::bfs_pred::Bfs::predecessors)
//! - [`dijkstra_pred::Dijkstra::predecessors`](algo::dijkstra_pred::Dijkstra::predecessors)
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
//!
//! ## Naming Conventions
//!
//! - `s` denotes a source vertex.
//! - `t` denotes a target vertex.
//! - `u` denotes a tail vertex or the first vertex in scope.
//! - `v` denotes a head vertex or the second vertex in scope.
//! - `w` denotes the weight of an arc.
//! - `x` denotes a tail vertex or the third vertex in scope.
//! - `y` denotes a head vertex or the fourth vertex in scope.
//!
//! [Adjacency List]: `adjacency_list::Digraph`
//! [Adjacency Matrix]: `adjacency_matrix::Digraph`
//! [Weighted Adjacency List]: `adjacency_list_weighted::Digraph`

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
pub mod gen;
pub mod op;
#[cfg(test)]
pub mod proptest_strategy;
