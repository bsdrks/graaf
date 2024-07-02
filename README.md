# Graaf &emsp; [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Work with directed graphs in Rust.

## Table of Contents

- [Installation](#installation)
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
- [Project goals](#project-goals)
- [Changelog](#changelog)
- [License](#license)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.64.1"
```

## Digraph Types

Graaf provides three representations of directed graphs.

- The [Adjacency List] is for unweighted and sparse digraphs.
- The [Adjacency Matrix] is for unweighted and dense digraphs.
- The [Weighted Adjacency List] is for weighted and sparse digraphs.

These types eagerly implement [digraph operations](#operations) and
[digraph algorithms](#algorithms).

## Creating Digraphs

The [`gen`] module provides four digraph generators.

- [`Complete`] generates a digraph with all possible arcs,
  excluding self-loops.
- [`Cycle`] generates a digraph with a cycle of a given length.
- [`Empty`] generates a digraph with no arcs.
- [`RandomTournament`] generates a random tournament.

---

| Generator            | [Adjacency List] | [Adjacency Matrix] | [Weighted Adjacency List] |
| :------------------- | :--------------- | :------------------ | :----------------------- |
| [`Complete`]         | Yes              | Yes                 | No                       |
| [`Cycle`]            | Yes              | Yes                 | No                       |
| [`Empty`]            | Yes              | Yes                 | Yes                      |
| [`RandomTournament`] | Yes              | Yes                 | No                       |

## Operations

The [`op`] module provides digraph operation traits. The [digraph
types](#digraph-types) implement these traits. One can implement these traits for
custom digraph types. Operations form the foundation for
[algorithms](#algorithms).

### Basic operations

[Individual digraph types](#digraph-types) implement the basic operations.

- [`AddArcWeighted`] adds an arc to a weighted digraph.
- [`AddArc`] adds an arc to an unweighted digraph.
- [`ArcWeight`] gets the weight of an arc.
- [`ArcsWeighted`] gets the arcs and their weights in a digraph.
- [`Arcs`] gets the arcs in a digraph.
- [`Converse`] gets the converse of a digraph.
- [`HasArc`] checks if an arc exists in a digraph.
- [`Indegree`] gets the indegree of a vertex.
- [`IsSimple`] checks if a digraph contains no loops or parallel arcs.
- [`Order`] gets the number of vertices.
- [`OutNeighborsWeighted`] gets the weighted out-neighbors of a vertex.
- [`OutNeighbors`] gets the out-neighbors of a vertex.
- [`Outdegree`] gets the outdegree of a vertex.
- [`RemoveArc`] removes an arc from a digraph.
- [`Size`] gets the number of arcs in a digraph.
- [`Vertices`] gets the vertices in a digraph.

---

| Operation                | [Adjacency List] | [Adjacency Matrix] | [Weighted Adjacency List] |
| :----------------------- | :--------------- | :----------------- | :------------------------ |
| [`AddArcWeighted`]       | No               | No                 | Yes                       |
| [`AddArc`]               | Yes              | Yes                | No                        |
| [`ArcWeight`]            | Yes              | Yes                | Yes                       |
| [`ArcsWeighted`]         | Yes              | Yes                | Yes                       |
| [`Arcs`]                 | Yes              | Yes                | Yes                       |
| [`Converse`]             | Yes              | Yes                | Yes                       |
| [`HasArc`]               | Yes              | Yes                | Yes                       |
| [`Indegree`]             | Yes              | Yes                | Yes                       |
| [`IsSimple`]             | Yes              | Yes                | Yes                       |
| [`Order`]                | Yes              | Yes                | Yes                       |
| [`OutNeighborsWeighted`] | Yes              | Yes                | Yes                       |
| [`OutNeighbors`]         | Yes              | Yes                | Yes                       |
| [`Outdegree`]            | Yes              | Yes                | Yes                       |
| [`RemoveArc`]            | Yes              | Yes                | Yes                       |
| [`Size`]                 | Yes              | Yes                | Yes                       |
| [`Vertices`]             | Yes              | Yes                | Yes                       |

### Extended operations

The extended traits derive their implementation from the basic
operations.

- [`Degree`] gets the degree of a vertex; requires [`Indegree`] `+`
  [`Outdegree`].
- [`HasEdge`] checks if an edge exists in a digraph; requires [`HasArc`].
- [`InNeighbors`] gets the in-neighbors of a vertex; requires [`Arcs`].
- [`IsBalanced`] checks if a digraph is balanced; requires [`Indegree`] `+`
  [`Outdegree`].
- [`IsComplete`] checks if a digraph is complete; requires [`HasEdge`] `+`
  [`Order`].
- [`IsIsolated`] checks if a vertex is isolated; requires [`Indegree`] `+`
  [`Outdegree`].
- [`IsOriented`] checks if a digraph is oriented; requires [`Arcs`] `+`
  [`HasArc`].
- [`IsPendant`] checks if a vertex is a pendant; requires [`Degree`].
- [`IsRegular`] checks if a digraph is regular; requires [`Indegree`] `+`
  [`Outdegree`] `+` [`Vertices`].
- [`IsSemicomplete`] checks if a digraph is semicomplete; requires [`HasArc`]
  `+` [`Order`].
- [`IsSubdigraph`] checks if a digraph is a subdigraph of another digraph;
  requires [`Arcs`] `+` [`HasArc`] `+` [`Vertices`]
- [`IsSuperdigraph`] checks if a digraph is a superdigraph of another digraph;
  requires [`Subdigraph`]
- [`IsSymmetric`] checks if a digraph is symmetric; requires [`Arcs`] `+`
  [`HasArc`].
- [`IsWalk`] checks if a sequence of vertices is a walk in a digraph; requires
  [`Arcs`].

---

| Operation          | [Adjacency List] | [Adjacency Matrix] | [Weighted Adjacency List] |
| :----------------- | :--------------- | :----------------- | :------------------------ |
| [`Degree`]         | Yes              | Yes                | Yes                       |
| [`HasEdge`]        | Yes              | Yes                | Yes                       |
| [`InNeighbors`]    | Yes              | Yes                | Yes                       |
| [`IsBalanced`]     | Yes              | Yes                | Yes                       |
| [`IsComplete`]     | Yes              | Yes                | Yes                       |
| [`IsIsolated`]     | Yes              | Yes                | Yes                       |
| [`IsOriented`]     | Yes              | Yes                | Yes                       |
| [`IsPendant`]      | Yes              | Yes                | Yes                       |
| [`IsRegular`]      | Yes              | Yes                | Yes                       |
| [`IsSemicomplete`] | Yes              | Yes                | Yes                       |
| [`IsSubdigraph`]   | Yes              | Yes                | Yes                       |
| [`IsSuperdigraph`] | Yes              | Yes                | Yes                       |
| [`IsSymmetric`]    | Yes              | Yes                | Yes                       |
| [`IsWalk`]         | Yes              | Yes                | Yes                       |

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
[Weighted Adjacency List]: https://docs.rs/graaf/latest/graaf/adjacency_list_weighted/digraph/struct.Digraph.html
[`AddArcWeighted`]: https://docs.rs/graaf/latest/graaf/op/add_arc_weighted/trait.AddArcWeighted.html
[`AddArc`]: https://docs.rs/graaf/latest/graaf/op/add_arc/trait.AddArc.html
[`ArcWeight`]: https://docs.rs/graaf/latest/graaf/op/arc_weight/trait.ArcWeight.html
[`ArcsWeighted`]: https://docs.rs/graaf/latest/graaf/op/arcs_weighted/trait.ArcsWeighted.html
[`Arcs`]: https://docs.rs/graaf/latest/graaf/op/arcs/trait.Arcs.html
[`Complete`]: https://docs.rs/graaf/latest/graaf/gen/complete/trait.Complete.html
[`Converse`]: https://docs.rs/graaf/latest/graaf/op/converse/trait.Converse.html
[`Cycle`]: https://docs.rs/graaf/latest/graaf/gen/cycle/trait.Cycle.html
[`Degree`]: https://docs.rs/graaf/latest/graaf/op/degree/trait.Degree.html
[`Empty`]: https://docs.rs/graaf/latest/graaf/gen/empty/trait.Empty.html
[`HasArc`]: https://docs.rs/graaf/latest/graaf/op/has_arc/trait.HasArc.html
[`HasEdge`]: https://docs.rs/graaf/latest/graaf/op/has_edge/trait.HasEdge.html
[`InNeighbors`]: https://docs.rs/graaf/latest/graaf/op/in_neighbors/trait.InNeighbors.html
[`Indegree`]: https://docs.rs/graaf/latest/graaf/op/indegree/trait.Indegree.html
[`IsBalanced`]: https://docs.rs/graaf/latest/graaf/op/is_balanced/trait.IsBalanced.html
[`IsComplete`]: https://docs.rs/graaf/latest/graaf/op/is_complete/trait.IsComplete.html
[`IsIsolated`]: https://docs.rs/graaf/latest/graaf/op/is_isolated/trait.IsIsolated.html
[`IsOriented`]: https://docs.rs/graaf/latest/graaf/op/is_oriented/trait.IsOriented.html
[`IsPendant`]: https://docs.rs/graaf/latest/graaf/op/is_pendant/trait.IsPendant.html
[`IsRegular`]: https://docs.rs/graaf/latest/graaf/op/is_regular/trait.IsRegular.html
[`IsSemicomplete`]: https://docs.rs/graaf/latest/graaf/op/is_semicomplete/trait.IsSemicomplete.html
[`IsSimple`]: https://docs.rs/graaf/latest/graaf/op/is_simple/trait.IsSimple.html
[`IsSubdigraph`]: https://docs.rs/graaf/latest/graaf/op/is_subdigraph/trait.IsSubdigraph.html
[`IsSuperdigraph`]: https://docs.rs/graaf/latest/graaf/op/is_superdigraph/trait.IsSuperdigraph.html
[`IsSymmetric`]: https://docs.rs/graaf/latest/graaf/op/is_symmetric/trait.IsSymmetric.html
[`IsWalk`]: https://docs.rs/graaf/latest/graaf/op/is_walk/trait.IsWalk.html
[`Order`]: https://docs.rs/graaf/latest/graaf/op/order/trait.Order.html
[`OutNeighborsWeighted`]: https://docs.rs/graaf/latest/graaf/op/out_neighbors_weighted/trait.OutNeighborsWeighted.html
[`OutNeighbors`]: https://docs.rs/graaf/latest/graaf/op/out_neighbors/trait.OutNeighbors.html
[`Outdegree`]: https://docs.rs/graaf/latest/graaf/op/outdegree/trait.Outdegree.html
[`RandomTournament`]: https://docs.rs/graaf/latest/graaf/gen/random_tournament/trait.RandomTournament.html
[`RemoveArc`]: https://docs.rs/graaf/latest/graaf/op/remove_arc/trait.RemoveArc.html
[`Size`]: https://docs.rs/graaf/latest/graaf/op/size/trait.Size.html
[`Vertices`]: https://docs.rs/graaf/latest/graaf/op/vertices/trait.Vertices.html
[`algo`]: https://docs.rs/graaf/latest/graaf/algo/index.html
[`bellman_ford_moore`]: https://docs.rs/graaf/latest/graaf/algo/bellman_ford_moore/index.html
[`bfs`]: https://docs.rs/graaf/latest/graaf/algo/bfs/index.html
[`dfs`]: https://docs.rs/graaf/latest/graaf/algo/dfs/index.html
[`dijkstra`]: https://docs.rs/graaf/latest/graaf/algo/dijkstra/index.html
[`floyd_warshall`]: https://docs.rs/gra
[`gen`]: https://docs.rs/graaf/latest/graaf/gen/index.html
[`op`]: https://docs.rs/graaf/latest/graaf/op/index.html

## Project goals

- A flexible API for digraph operations
- A comprehensive set of algorithms
- Generators for common digraphs
- Competitive performance
- Full documentation
- Extensive property tests
- Complete unit test and benchmark coverage

## Changelog

See [CHANGELOG.md] for a list of changes.

## License

Licensed under [Apache License, Version 2.0] or [MIT license] at your option.

[CHANGELOG.md]: https://github.com/bsdrks/graaf/blob/main/CHANGELOG.md
[Apache License, Version 2.0]: LICENSE-APACHE
[MIT license]: LICENSE-MIT
