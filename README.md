# Graaf &emsp; [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=integrate-coveralls)](https://coveralls.io/github/bsdrks/graaf?branch=integrate-coveralls)

**WARNING: this crate is pre-alpha**

Functions and types for working with graphs

This crate builds on `nightly`. This will change in version `1.0.0`.

## Algorithms

- `algo::bfs::min_distances_single_source` calculates the minimum distances from the source verticex to all vertices.
- `algo::bfs::min_distances` calculates the minimum distances from multiple source vertices to all vertices.
- `algo::bfs::shortest_paths` calculates the shortest paths from multiple source vertices to all vertices.
- `algo::dijkstra::unweighted::min_distances_single_source` calculates the minimum distances from the source vertex to all vertices.
- `algo::dijkstra::unweighted::min_distances` calculates the minimum distances from multiple source vertices to all vertices.
- `algo::dijkstra::unweighted::shortest_paths` calculates the shortest paths from multiple source vertices to all vertices.
- `algo::dijkstra::weighted::min_distances_single_source` calculates the minimum distances from the source vertex to all vertices.
- `algo::dijkstra::weighted::min_distances` calculates the minimum distances from multiple source vertices to all vertices.

## Graph operation traits

These traits are implemented for various graph representations built from standard library containers.

- `AddEdge` adds an edge to an unweighted graph.
- `AddWeightedEdge` adds an edge to a weighted graph.
- `CountAllEdges` counts all edges in a graph.
- `CountAllVertices` counts all vertices in a graph.
- `EdgeWeight` gets the weight of an edge.
- `Indegree` returns the indegree of a vertex.
- `IsEdge` returns whether an edge exists between two vertices.
- `IterAllEdges` iterates over all unweighted edges in a graph.
- `IterAllWeightedEdges` iterates over all weighted edges in a graph.
- `IterEdges` iterates over all unweighted edges of a source vertex.
- `IterVertices` iterates over all vertices in a graph.
- `IterWeightedEdges` iterates over all weighted edges of a source vertex.
- `Outdegree` returns the outdegree of a vertex.
- `RemoveEdge` removes an edge from a graph.
- `VertexWeight` returns the weight of a vertex.

## Graph representations

### Adjacency list

#### Unweighted

- `Vec<Vec<usize>>`
- `Vec<HashSet<usize>>`
- `[Vec<usize>]`
- `[HashSet<usize>]`
- `HashMap<usize, Vec<usize>>`
- `HashMap<usize, HashSet<usize>>`

#### Weighted

- `Vec<Vec<(usize, W)>>`
- `Vec<HashSet<(usize, W)>>`
- `Vec<HashMap<usize, W>>`
- `[Vec<(usize, W)>]`
- `[HashSet<(usize, W)>]`
- `[HashMap<usize, W>]`
- `HashMap<usize, Vec<(usize, W)>>`
- `HashMap<usize, HashSet<(usize, W)>>`
- `HashMap<usize, HashMap<usize, W>>`

### Adjacency matrix

#### Unweighted

- `AdjacencyMatrix`: an adjacency matrix representation of an unweighted directed graph stored as a bit array.

### Edge list

#### Unweighted

- `Vec<(usize, usize)>`
- `[(usize, usize)]`
- `HashSet<(usize, usize)>`

#### Weighted

- `Vec<(usize, usize, W)>`
- `[(usize, usize, W)]`
- `HashSet<(usize, usize, W)>`
