//! # Graaf
//!
//! Work with directed graphs in Rust.
//!
//! # Table of Contents
//!
//! - [Digraph Types](#digraph-types)
//! - [Creating Digraphs](#creating-digraphs)
//! - [Operations](#operations)
//!    - [Basic Operations](#basic-operations)
//!    - [Extended Operations](#extended-operations)
//! - [Algorithms](#algorithms)
//!    - [Bellman-Ford-Moore](#bellman-ford-moore)
//!    - [Breadth-First Search (BFS)](#breadth-first-search-bfs)
//!    - [Depth-First Search (DFS)](#depth-first-search-dfs)
//!    - [Dijkstra](#dijkstra)
//!    - [Floyd-Warshall](#floyd-warshall)
//!    - [Tarjan](#tarjan)
//!    - [Types](#types)
//!       - [Breath-First Tree](#breadth-first-tree)
//!       - [Distance Matrix](#distance-matrix)
//! - [Naming Conventions](#naming-conventions)
//! - [Project Goals](#project-goals)
//!
//! # Digraph Types
//!
//! Graaf provides three representations of directed graphs.
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
//! Graaf provides eight digraph generators.
//!
//! - [`Biclique`](gen::Biclique) generates a complete bipartite digraph.
//! - [`Circuit`](gen::Circuit) generates a circuit digraph.
//! - [`Complete`](gen::Complete) generates a complete digraph.
//! - [`Cycle`](gen::Cycle) generates a bidirectional circuit.
//! - [`Empty`](gen::Empty) generates a digraph with no arcs.
//! - [`RandomTournament`](gen::RandomTournament) generates a random tournament.
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
//! ## Basic Operations
//!
//! [Individual digraph types](#digraph-types) implement the basic operations.
//!
//! - [`AddArcWeighted`](op::AddArcWeighted) adds an arc to an arc-weighted
//!   digraph.
//! - [`AddArc`](op::AddArc) adds an arc to an unweighted digraph.
//! - [`ArcWeight`](op::ArcWeight) returns the weight of an arc.
//! - [`ArcsWeighted`](op::ArcsWeighted) returns the arcs and their weights in a
//!   digraph.
//! - [`Arcs`](op::Arcs) returns the arcs in a digraph.
//! - [`Converse`](op::Converse) returns the converse of a digraph.
//! - [`HasArc`](op::HasArc) checks if an arc exists in a digraph.
//! - [`Indegree`](op::Indegree) returns the indegree of a vertex.
//! - [`IsSimple`](op::IsSimple) checks if a digraph contains no loops or
//!   parallel arcs.
//! - [`Order`](op::Order) returns the number of vertices.
//! - [`OutNeighborsWeighted`](op::OutNeighborsWeighted) returns the weighted
//!   out-neighbors of a vertex.
//! - [`OutNeighbors`](op::OutNeighbors) returns the out-neighbors of a vertex.
//! - [`Outdegree`](op::Outdegree) returns the outdegree of a vertex.
//! - [`RemoveArc`](op::RemoveArc) removes an arc from a digraph.
//! - [`Size`](op::Size) returns the number of arcs in a digraph.
//! - [`Vertices`](op::Vertices) returns the vertices in a digraph.
//!
//! ## Extended Operations
//!
//! The extended traits derive their implementation from the basic
//! operations.
//!
//! - [`Complement`](op::Complement) returns the complement of a digraph.
//! - [`Degree`](op::Degree) returns the degree of a vertex.
//! - [`DegreeSequence`](op::DegreeSequence) returns the degree sequence of a
//!   digraph.
//! - [`HasEdge`](op::HasEdge) checks if an edge exists in a digraph.
//! - [`InNeighbors`](op::InNeighbors) returns the in-neighbors of a vertex.
//! - [`IsBalanced`](op::IsBalanced) checks if a digraph is balanced.
//! - [`IsComplete`](op::IsComplete) checks if a digraph is complete.
//! - [`IsIsolated`](op::IsIsolated) checks if a vertex is isolated.
//! - [`IsOriented`](op::IsOriented) checks if a digraph is oriented.
//! - [`IsPendant`](op::IsPendant) checks if a vertex is a pendant.
//! - [`IsRegular`](op::IsRegular) checks if a digraph is regular.
//! - [`IsSemicomplete`](op::IsSemicomplete) checks if a digraph is
//!   semicomplete.
//! - [`IsSpanningSubdigraph`](op::IsSpanningSubdigraph) checks if a digraph is
//!   a spanning subdigraph.
//! - [`IsSubdigraph`](op::IsSubdigraph) checks if a digraph is a subdigraph.
//! - [`IsSuperdigraph`](op::IsSuperdigraph) checks if a digraph is a
//!   superdigraph.
//! - [`IsSymmetric`](op::IsSymmetric) checks if a digraph is symmetric.
//! - [`IsTournament`](op::IsTournament) checks if a digraph is a tournament.
//! - [`IsWalk`](op::IsWalk) checks if a sequence of vertices is a walk in a
//!   digraph.
//!
//! # Algorithms
//!
//! The [`algo`] module provides digraph algorithms.
//!
//! ## Bellman-Ford-Moore
//!
//! The Bellman-Ford-Moore algorithm finds the shortest paths in an arc-weighted
//! digraph with negative weights.
//!
//! - [`single_source_distances`](algo::bellman_ford_moore::single_source_distances) finds the shortest distances.
//!
//! ## Breadth-First Search (BFS)
//!
//! A breadth-first search explores the vertices of an unweighted digraph in
//! order of their distance from a source.
//!
//! These functions start from one or more source vertices and allow a custom
//! step function, target predicate, distance array, breadth-first tree, and
//! queue, where applicable.
//!
//! - [`distances`](algo::bfs::distances) finds the shortest distances.
//! - [`predecessors`](algo::bfs::predecessors) finds the predecessors.
//! - [`shortest_path`](algo::bfs::shortest_path) finds the shortest path.
//!
//! These functions start from one source vertex.
//!
//! - [`single_source_distances`](algo::bfs::single_source_distances) finds the
//!   shortest distances.
//! - [`single_source_predecessors`](algo::bfs::single_source_predecessors)
//!   finds the predecessors.
//! - [`single_pair_shortest_path`](algo::bfs::single_pair_shortest_path) finds
//!   the shortest path.
//!
//! ## Depth-First Search (DFS)
//!
//! A depth-first search explores the vertices of an unweighted digraph in order
//! of their depth from a source.
//!
//! - [`dfsa`](algo::dfs::dfsa) traverses the digraph.
//! - [`dfsa_predecessors`](algo::dfs::dfsa_predecessors) finds the
//!   predecessors.
//! - [`acyclic_ordering`](algo::dfs::acyclic_ordering) generates an acyclic
//!   ordering.
//!
//! ## Dijkstra
//!
//! Dijkstra's algorithm finds the shortest paths from one or more source
//! vertices in an arc-weighted digraph.
//!
//! These functions start from one or more source vertices and allow a custom
//! step function, target predicate, distance array, and heap, where
//! applicable.
//!
//! - [`distances`](algo::dijkstra::distances) finds the shortest distances.
//! - [`predecessors`](algo::dijkstra::predecessors) finds the predecessors.
//! - [`shortest_path`](algo::dijkstra::shortest_path) finds the shortest path.
//!
//! These functions start from one source vertex.
//!
//! - [`single_source_distances`](algo::dijkstra::single_source_distances) finds
//!   the shortest distances.
//! - [`single_source_predecessors`](algo::dijkstra::single_source_predecessors)
//!   finds the predecessors.
//! - [`single_pair_shortest_path`](algo::dijkstra::single_pair_shortest_path)
//!   finds the shortest path.
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
//! ## Types
//!
//! These types are produced by the algorithms.
//!
//! ### Breadth-First Tree
//!
//! A breadth-first tree is the result of a breadth-first search and contains
//! the predecessors of the vertices on the shortest paths.
//!
//! - [`search`](algo::BreadthFirstTree::search) finds a vertex by value.
//! - [`search_by`](algo::BreadthFirstTree::search_by) finds a vertex by
//!   predicate.
//!
//! These functions produce a breadth-first tree.
//!
//! - [`bfs::single_source_predecessors`](algo::bfs::single_source_predecessors)
//! - [`bfs::predecessors`](algo::bfs::predecessors)
//! - [`dijkstra::single_source_predecessors`](algo::dijkstra::single_source_predecessors)
//! - [`dijkstra::predecessors`](algo::dijkstra::predecessors)
//!
//! ### Distance Matrix
//!
//! A distance matrix contains the shortest distances between all pairs of
//! vertices in a digraph.
//!
//! - [`center`](algo::DistanceMatrix::center) finds the center of the digraph.
//! - [`diameter`](algo::DistanceMatrix::diameter) finds the diameter of the
//!   digraph.
//! - [`eccentricities`](algo::DistanceMatrix::eccentricities) returns the
//!   eccentricities of the vertices.
//! - [`is_connected`](algo::DistanceMatrix::is_connected) checks if the digraph
//!   is connected.
//! - [`periphery`](algo::DistanceMatrix::periphery) finds the periphery of the
//!   digraph.
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
//! ## Project Goals
//!
//! - A flexible API for digraph operations
//! - A comprehensive set of algorithms
//! - Generators for common digraphs
//! - Competitive performance
//! - Full documentation
//! - Extensive property tests
//! - Complete unit test and benchmark coverage
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
