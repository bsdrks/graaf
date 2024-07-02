# Graaf &emsp; [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Work with directed graphs in Rust.

## Table of Contents

- [Digraph Types](#digraph-types)
- [Creating Digraphs](#creating-digraphs)
- [Operations](#operations)
  - [Basic operations](#basic-operations)
  - [Extended operations](#extended-operations)
- [Algorithms](#algorithms)
  - [Bellman-Ford-Moore](#bellman-ford-moore)
  - [Breadth-first search (BFS)](#breadth-first-search-bfs)
  - [Depth-first search (DFS)](#depth-first-search-dfs)
  - [Dijkstra's algorithm](#dijkstras-algorithm)
  - [Floyd-Warshall algorithm](#floyd-warshall)
  - [Breath-first tree](#breadth-first-tree)
  - [Distance matrix](#distance-matrix)

## Digraph Types

Graaf provides three representations of directed graphs.

- The [Adjacency List] is for unweighted and sparse digraphs.
- The [Adjacency Matrix] is for unweighted and dense digraphs.
- The [Weighted Adjacency List] is for weighted and sparse digraphs.

These types eagerly implement [digraph operations](#operations) and
[digraph algorithms](#algorithms).

## Creating Digraphs

Graaf provides four digraph generators.

- [Complete] generates a digraph with all possible arcs,
  excluding self-loops.
- [`Cycle`](gen::Cycle) generates a digraph with a cycle of a given length.
- [`Empty`](gen::Empty) generates a digraph with no arcs.
- [`RandomTournament`](gen::RandomTournament) generates a random tournament.

### Implementations

| Generator                                   | [Adjacency List] | [Adjacency Matrix] | [Weighted Adjacency List] |
| :------------------------------------------ | :--------------- | :------------------ | :----------------------- |
| [Complete]                 | Yes              | Yes                 | No                       |
| [`Cycle`](gen::Cycle)                       | Yes              | Yes                 | No                       |
| [`Empty`](gen::Empty)                       | Yes              | Yes                 | Yes                      |
| [`RandomTournament`](gen::RandomTournament) | Yes              | Yes                 | No                       |

## Operations

The [`op`] module provides digraph operation traits. The [digraph
types](#digraph-types) implement these traits. One can implement these traits for
custom digraph types. Operations form the foundation for
[algorithms](#algorithms).

### Basic operations

[Individual digraph types](#digraph-types) implement the basic operations.

- [`AddArcWeighted`](op::AddArcWeighted) adds an arc to a weighted digraph.
- [`AddArc`](op::AddArc) adds an arc to an unweighted digraph.
- [`ArcWeight`](op::ArcWeight) gets the weight of an arc.
- [`ArcsWeighted`](op::ArcsWeighted) gets the arcs and their weights in a
  digraph.
- [`Arcs`](op::Arcs) gets the arcs in a digraph.
- [`Converse`](op::Converse) gets the converse of a digraph.
- [`HasArc`](op::HasArc) checks if an arc exists in a digraph.
- [`Indegree`](op::Indegree) gets the indegree of a vertex.
- [`IsSimple`](op::IsSimple) checks if a digraph contains no loops or
  parallel arcs.
- [`Order`](op::Order) gets the number of vertices.
- [`OutNeighborsWeighted`](op::OutNeighborsWeighted) gets the weighted
  out-neighbors of a vertex.
- [`OutNeighbors`](op::OutNeighbors) gets the out-neighbors of a vertex.
- [`Outdegree`](op::Outdegree) gets the outdegree of a vertex.
- [`RemoveArc`](op::RemoveArc) removes an arc from a digraph.
- [`Size`](op::Size) gets the number of arcs in a digraph.
- [`Vertices`](op::Vertices) gets the vertices in a digraph.

Implementations

| Operation                                          | [Adjacency List] | [Adjacency Matrix] | [Weighted Adjacency List] |
| :------------------------------------------------- | :--------------- | :----------------- | :------------------------ |
| [`AddArcWeighted`](op::AddArcWeighted)             | No               | No                 | Yes                       |
| [`AddArc`](op::AddArc)                             | Yes              | Yes                | No                        |
| [`ArcWeight`](op::ArcWeight)                       | Yes              | Yes                | Yes                       |
| [`ArcsWeighted`](op::ArcsWeighted)                 | Yes              | Yes                | Yes                       |
| [`Arcs`](op::Arcs)                                 | Yes              | Yes                | Yes                       |
| [`Converse`](op::Converse)                         | Yes              | Yes                | Yes                       |
| [`HasArc`](op::HasArc)                             | Yes              | Yes                | Yes                       |
| [`Indegree`](op::Indegree)                         | Yes              | Yes                | Yes                       |
| [`IsSimple`](op::IsSimple)                         | Yes              | Yes                | Yes                       |
| [`Order`](op::Order)                               | Yes              | Yes                | Yes                       |
| [`OutNeighborsWeighted`](op::OutNeighborsWeighted) | Yes              | Yes                | Yes                       |
| [`OutNeighbors`](op::OutNeighbors)                 | Yes              | Yes                | Yes                       |
| [`Outdegree`](op::Outdegree)                       | Yes              | Yes                | Yes                       |
| [`RemoveArc`](op::RemoveArc)                       | Yes              | Yes                | Yes                       |
| [`Size`](op::Size)                                 | Yes              | Yes                | Yes                       |
| [`Vertices`](op::Vertices)                         | Yes              | Yes                | Yes                       |

### Extended operations

The extended traits derive their implementation from the basic
operations.

- [`Degree`](op::Degree) gets the degree of a vertex. Requires
  [`Indegree`](op::Indegree) `+` [`Outdegree`](op::Outdegree).
- [`HasEdge`](op::HasEdge) checks if an edge exists in a digraph. Requires
  [`HasArc`](op::HasArc).
- [`InNeighbors`](op::InNeighbors) gets the in-neighbors of a vertex.
  Requires [`Arcs`](op::Arcs).
- [`IsBalanced`](op::IsBalanced) checks if a digraph is balanced. Requires
  [`Indegree`](op::Indegree) `+` [`Outdegree`](op::Outdegree).
- [`IsComplete`](op::IsComplete) checks if a digraph is complete. Requires
  [`HasEdge`](op::HasEdge) `+` [`Order`](op::Order).
- [`IsIsolated`](op::IsIsolated) checks if a vertex is isolated. Requires
  [`Indegree`](op::Indegree) `+` [`Outdegree`](op::Outdegree).
- [`IsOriented`](op::IsOriented) checks if a digraph is oriented. Requires
  [`Arcs`](op::Arcs) `+` [`HasArc`](op::HasArc).
- [`IsPendant`](op::IsPendant) checks if a vertex is a pendant. Requires
  [`Degree`](op::Degree).
- [`IsRegular`](op::IsRegular) checks if a digraph is regular. Requires
  [`Indegree`](op::Indegree) `+` [`Outdegree`](op::Outdegree) `+`
  [`Vertices`](op::Vertices).
- [`IsSemicomplete`](op::IsSemicomplete) checks if a digraph is
  semicomplete. Requires [`HasArc`](op::HasArc) `+` [`Order`](op::Order).
- [`IsSubdigraph`](op::IsSubdigraph) checks if a digraph is a subdigraph of
  another digraph. Requires [`Arcs`](op::Arcs) `+` [`HasArc`](op::HasArc)
  `+` [`Vertices`](op::Vertices).
- [`IsSuperdigraph`](op::IsSuperdigraph) checks if a digraph is a
  superdigraph of another digraph. Requires
  [`Subdigraph`](op::IsSubdigraph).
- [`IsSymmetric`](op::IsSymmetric) checks if a digraph is symmetric.
  Requires [`Arcs`](op::Arcs) `+` [`HasArc`](op::HasArc).
- [`IsWalk`](op::IsWalk) checks if a sequence of vertices is a walk in a
  digraph. Requires [`Arcs`](op::Arcs).

Implementations

| Operation                              | [Adjacency List] | [Adjacency Matrix] | [Weighted Adjacency List] |
| :------------------------------------- | :--------------- | :----------------- | :------------------------ |
| [`Degree`](op::Degree)                 | Yes              | Yes                | Yes                       |
| [`HasEdge`](op::HasEdge)               | Yes              | Yes                | Yes                       |
| [`InNeighbors`](op::InNeighbors)       | Yes              | Yes                | Yes                       |
| [`IsBalanced`](op::IsBalanced)         | Yes              | Yes                | Yes                       |
| [`IsComplete`](op::IsComplete)         | Yes              | Yes                | Yes                       |
| [`IsIsolated`](op::IsIsolated)         | Yes              | Yes                | Yes                       |
| [`IsOriented`](op::IsOriented)         | Yes              | Yes                | Yes                       |
| [`IsPendant`](op::IsPendant)           | Yes              | Yes                | Yes                       |
| [`IsRegular`](op::IsRegular)           | Yes              | Yes                | Yes                       |
| [`IsSemicomplete`](op::IsSemicomplete) | Yes              | Yes                | Yes                       |
| [`IsSubdigraph`](op::IsSubdigraph)     | Yes              | Yes                | Yes                       |
| [`IsSuperdigraph`](op::IsSuperdigraph) | Yes              | Yes                | Yes                       |
| [`IsSymmetric`](op::IsSymmetric)       | Yes              | Yes                | Yes                       |
| [`IsWalk`](op::IsWalk)                 | Yes              | Yes                | Yes                       |

## Algorithms

The [`algo`] module provides digraph algorithms.

### Bellman-Ford-Moore

[`bellman_ford_moore`](algo::bellman_ford_moore) finds the shortest paths
from one source vertex in a weighted digraph with negative weights.

- [`single_source_distances`](algo::bellman_ford_moore::single_source_distances)
  finds the shortest distances from a source vertex to all other vertices.

### Breadth-first search (BFS)

[`bfs`](algo::bfs) explores the vertices of an unweighted digraph in order
of their distance from a source.

These functions start from one or more source vertices and allow a custom
step function, target predicate, distance array, breadth-first tree, and
queue value, where applicable.

- [`distances`](algo::bfs::distances) finds the shortest distances to all
  other vertices.
- [`predecessors`](algo::bfs::predecessors) finds the predecessors of the
  vertices on the shortest paths.
- [`shortest_path`](algo::bfs::shortest_path) finds the shortest path to a
  target vertex.

These algorithms start from one source vertex.

- [`single_source_distances`](algo::bfs::single_source_distances) finds the
  distances to all other vertices.
- [`single_source_predecessors`](algo::bfs::single_source_predecessors)
  finds the predecessors of the vertices on the shortest paths.
- [`single_pair_shortest_path`](algo::bfs::single_pair_shortest_path) finds
  the shortest path between two vertices.

### Depth-first search (DFS)

[`dfs`](algo::dfs) explores the vertices of an unweighted digraph in order
of their depth from a source.

- [`dfsa`](algo::dfs::dfsa) traverses the digraph, collecting an acyclic
  ordering and the times of each vertex's first and last visit.
- [`dfsa_predecessors`](algo::dfs::dfsa_predecessors) collects an acyclic
  ordering, the predecessors, and the times of each vertex's first and last
  visit.
- [`acyclic_ordering`](algo::dfs::acyclic_ordering) generates an acyclic
  ordering of the vertices.

### Dijkstra's algorithm

[`dijkstra`](algo::dijkstra) finds the shortest paths from one or more
source vertices in a weighted digraph.

These functions start from one or more source vertices and allow a custom
step function, target predicate, distance array, and heap value, where
applicable.

- [`distances`](algo::dijkstra::distances) finds the shortest distances to
  all other vertices.
- [`predecessors`](algo::dijkstra::predecessors) finds the predecessors of
  the vertices on the shortest paths.
- [`shortest_path`](algo::dijkstra::shortest_path) finds the shortest path
  to a target vertex.

These algorithms start from one source vertex.

- [`single_source_distances`](algo::dijkstra::single_source_distances) finds
  the shortest distances to all other vertices.
- [`single_source_predecessors`](algo::dijkstra::single_source_predecessors)
  finds the predecessors of the vertices on the shortest paths.
- [`single_pair_shortest_path`](algo::dijkstra::single_pair_shortest_path)
  finds the shortest path between two vertices.

### Floyd-Warshall

[`floyd_warshall`](algo::floyd_warshall) algorithm finds the shortest paths
between all pairs of vertices in a weighted digraph.

- [`distances`](algo::floyd_warshall::distances) finds the shortest
  distances between all pairs of vertices.

### Breadth-first tree

A [`BreadthFirstTree`](algo::breadth_first_tree::BreadthFirstTree) tree
contains the predecessors of the vertices in a breadth-first search.

- [`search`](algo::breadth_first_tree::BreadthFirstTree::search) returns the
  path to a target vertex.
- [`search_by`](algo::breadth_first_tree::BreadthFirstTree::search_by)
  returns the path to a vertex that satisfies a predicate.

These functions produce a
[`BreadthFirstTree`](algo::breadth_first_tree::BreadthFirstTree).

- [`bfs::single_source_predecessors`](algo::bfs::single_source_predecessors)
- [`bfs::predecessors`](algo::bfs::predecessors)
- [`dijkstra::single_source_predecessors`](algo::dijkstra::single_source_predecessors)
- [`dijkstra::predecessors`](algo::dijkstra::predecessors)

### Distance matrix

A [`DistanceMatrix`](algo::distance_matrix::DistanceMatrix) contains the
shortest distances between all pairs of vertices in a digraph.

- [`center`](algo::distance_matrix::DistanceMatrix::center) finds the center
  of the digraph.
- [`diameter`](algo::distance_matrix::DistanceMatrix::diameter) finds the
  diameter of the digraph.
- [`eccentricities`](algo::distance_matrix::DistanceMatrix::eccentricities)
  returns the eccentricities of the vertices.
- [`is_connected`](algo::distance_matrix::DistanceMatrix::is_connected)
  checks if the digraph is connected.

[Adjacency List]: https://docs.rs/graaf/latest/graaf/adjacency_list/digraph/struct.Digraph.html
[Adjacency Matrix]: https://docs.rs/graaf/latest/graaf/adjacency_matrix/digraph/struct.Digraph.html
[Complete]: https://docs.rs/graaf/latest/graaf/gen/complete/trait.Complete.html
[Weighted Adjacency List]: https://docs.rs/graaf/latest/graaf/adjacency_list_weighted/digraph/struct.Digraph.html

## Project goals

- A flexible API for digraph operations
- A comprehensive set of algorithms
- Generators for common digraphs
- Competitive performance
- Full documentation
- Extensive property tests
- Complete unit test and benchmark coverage

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.64.1"
```

## Changelog

See [CHANGELOG.md] for a list of changes.

## License

Licensed under [Apache License, Version 2.0] or [MIT license] at your option.

[CHANGELOG.md]: https://github.com/bsdrks/graaf/blob/main/CHANGELOG.md
[Apache License, Version 2.0]: LICENSE-APACHE
[MIT license]: LICENSE-MIT
