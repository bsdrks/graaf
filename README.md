# Graaf &emsp; [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Work with directed graphs in Rust.

## Table of Contents

- [Installation](#installation)
- [Digraph Types](#digraph-types)
- [Creating Digraphs](#creating-digraphs)
- [Operations](#operations)
  - [Basic Operations](#basic-operations)
  - [Extended Operations](#extended-operations)
- [Algorithms](#algorithms)
  - [Bellman-Ford-Moore](#bellman-ford-moore)
  - [Breadth-First Search (BFS)](#breadth-first-search-bfs)
  - [Depth-First Search (DFS)](#depth-first-search-dfs)
  - [Dijkstra's Algorithm](#dijkstras-algorithm)
  - [Floyd-Warshall Algorithm](#floyd-warshall)
  - [Breath-First Tree](#breadth-first-tree)
  - [Distance Matrix](#distance-matrix)
- [Project Goals](#project-goals)
- [Changelog](#changelog)
- [License](#license)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.64.6"
```

## Digraph Types

Graaf provides three representations of directed graphs.

- The [Adjacency List] type represents unweighted sparse digraphs.
- The [Adjacency Matrix] type represents unweighted dense digraphs.
- The [Weighted Adjacency List] type represents weighted sparse digraphs.

These types eagerly implement [digraph operations](#operations) and [digraph algorithms](#algorithms).

## Creating Digraphs

The [`gen`] module provides four digraph generators.

- The [`Complete`] trait generates a digraph in which an arc connects every ordered pair of distinct vertices.
- The [`Cycle`] trait generates a digraph with a cycle of a given length.
- The [`Empty`] trait generates a digraph with no arcs.
- The [`RandomTournament`] trait generates a random digraph in which an arc connects every unordered pair of distinct vertices.

## Operations

The [`op`] module provides digraph operation traits. The [digraph types](#digraph-types) implement these traits. One can implement these traits for custom digraph types. Operations form the foundation for [algorithms](#algorithms).

### Basic Operations

[Individual digraph types](#digraph-types) implement the basic operations.

- The [`AddArcWeighted`] trait adds an arc to a weighted digraph.
- The [`AddArc`] trait adds an arc to an unweighted digraph.
- The [`ArcWeight`] trait returns the weight of an arc.
- The [`ArcsWeighted`] trait returns the arcs and their weights in a digraph.
- The [`Arcs`] trait returns the arcs in a digraph.
- The [`Converse`] trait returns the converse of a digraph.
- The [`HasArc`] trait checks if an arc exists in a digraph.
- The [`Indegree`] trait returns the indegree of a vertex.
- The [`IsSimple`] trait checks if a digraph contains no loops or parallel arcs.
- The [`Order`] trait returns the number of vertices.
- The [`OutNeighborsWeighted`] trait returns the weighted out-neighbors of a vertex.
- The [`OutNeighbors`] trait returns the out-neighbors of a vertex.
- The [`Outdegree`] trait returns the outdegree of a vertex.
- The [`RemoveArc`] trait removes an arc from a digraph.
- The [`Size`] trait returns the number of arcs in a digraph.
- The [`Vertices`] trait returns the vertices in a digraph.

### Extended Operations

The extended traits derive their implementation from the basic operations.

- The [`Degree`] trait returns the degree of a vertex.
- The [`HasEdge`] trait checks if an edge exists in a digraph.
- The [`InNeighbors`] trait returns the in-neighbors of a vertex.
- The [`IsBalanced`] trait checks if a digraph is balanced.
- The [`IsComplete`] trait checks if a digraph is complete.
- The [`IsIsolated`] trait checks if a vertex is isolated.
- The [`IsOriented`] trait checks if a digraph is oriented.
- The [`IsPendant`] trait checks if a vertex is a pendant.
- The [`IsRegular`] trait checks if a digraph is regular.
- The [`IsSemicomplete`] trait checks if a digraph is semicomplete.
- The [`IsSubdigraph`] trait checks if a digraph is a subdigraph of another digraph.
- The [`IsSuperdigraph`] trait checks if a digraph is a superdigraph of another digraph.
- The [`IsSymmetric`] trait checks if a digraph is symmetric.
- The [`IsWalk`] trait checks if a sequence of vertices is a walk in a digraph.

## Algorithms

The [`algo`] module provides digraph algorithms.

### Bellman-Ford-Moore

The Bellman-Ford-Moore algorithm finds the shortest paths in a weighted digraph with negative weights.

- The [`single_source_distances`](https://docs.rs/graaf/latest/graaf/algo/bellman_ford_moore/fn.single_source_distances.html) function finds the shortest distances from one source vertex to all other vertices.

### Breadth-First Search (BFS)

A breadth-first search explores the vertices of an unweighted digraph in order of their distance from a source.

These functions start from one or more source vertices and allow a custom step function, target predicate, distance array, breadth-first tree, and queue value, where applicable.

- The [`distances`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.distances.html) function finds the shortest distances to all other vertices.
- The [`predecessors`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.predecessors.html) function finds the predecessors of the vertices on the shortest paths.
- The [`shortest_path`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.shortest_path.html) function finds the shortest path to a target vertex.

These functions start from one source vertex.

- The [`single_source_distances`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.single_source_distances.html) function finds the distances to all other vertices.
- The [`single_source_predecessors`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.single_source_predecessors.html) function finds the predecessors on the shortest paths.
- The [`single_pair_shortest_path`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.single_pair_shortest_path.html) function finds the shortest path between two vertices.

### Depth-First Search (DFS)

A depth-first search explores the vertices of an unweighted digraph in order of their depth from a source.

- The [`dfsa`](https://docs.rs/graaf/latest/graaf/algo/dfs/fn.dfsa.html) function traverses the digraph, collecting an acyclic ordering and the times of each vertex's first and last visit.
- The [`dfsa_predecessors`](https://docs.rs/graaf/latest/graaf/algo/dfs/fn.dfsa_predecessors.html) function collects an acyclic ordering, the predecessors, and the times of each vertex's first and last visit.
- The [`acyclic_ordering`](https://docs.rs/graaf/latest/graaf/algo/dfs/fn.acyclic_ordering.html) function generates an acyclic ordering of the vertices.

### Dijkstra's Algorithm

Dijkstra's algorithm finds the shortest paths from one or more source vertices in a weighted digraph.

These functions start from one or more source vertices and allow a custom step function, target predicate, distance array, and heap value, where applicable.

- The [`distances`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.distances.html) function finds the shortest distances to all other vertices.
- The [`predecessors`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.predecessors.html) function finds the predecessors of the vertices on the shortest paths.
- The [`shortest_path`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.shortest_path.html) function finds the shortest path to a target vertex.

These functions start from one source vertex.

- The [`single_source_distances`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.single_source_distances.html) function finds the shortest distances to all other vertices.
- The [`single_source_predecessors`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.single_source_predecessors.html) function finds the predecessors of the vertices on the shortest paths.
- The [`single_pair_shortest_path`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.single_pair_shortest_path.html) function finds the shortest path between two vertices.

### Floyd-Warshall

The Floyd-Warshall algorithm finds the shortest paths between all pairs of vertices in a weighted digraph.

- The [`distances`](https://docs.rs/graaf/latest/graaf/algo/floyd_warshall/fn.distances.html) function finds the shortest distances between all pairs of vertices.

### Breadth-First Tree

A breadth-first-tree is the result of a breadth-first search.

- The [`search`](https://docs.rs/graaf/latest/graaf/algo/breadth_first_tree/struct.BreadthFirstTree.html#method.search) method finds the path to a target vertex.
- The [`search_by`](https://docs.rs/graaf/latest/graaf/algo/breadth_first_tree/struct.BreadthFirstTree.html#method.search_by) method finds the path to a vertex that satisfies a predicate.

These functions produce a [`BreadthFirstTree`](https://docs.rs/graaf/latest/graaf/algo/breadth_first_tree/struct.BreadthFirstTree.html).

- [`bfs::single_source_predecessors`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.single_source_predecessors.html)
- [`bfs::predecessors`](https://docs.rs/graaf/latest/graaf/algo/bfs/fn.predecessors.html)
- [`dijkstra::single_source_predecessors`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.single_source_predecessors.html)
- [`dijkstra::predecessors`](https://docs.rs/graaf/latest/graaf/algo/dijkstra/fn.predecessors.html)

### Distance Matrix

A distance matrix contains the shortest distances between all pairs of vertices in a digraph.

- The [`center`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.center) method finds the center of the digraph.
- The [`diameter`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.diameter) method finds the diameter of the digraph.
- The [`eccentricities`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.eccentricities) method returns the eccentricities of the vertices.
- The [`is_connected`](https://docs.rs/graaf/latest/graaf/algo/distance_matrix/struct.DistanceMatrix.html#method.is_connected) method checks if the digraph is connected.

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
