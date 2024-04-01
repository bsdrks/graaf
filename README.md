[![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions)
[![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf)
[![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf)

# Graaf

Functions and types for working with graphs

**WARNING: this crate is pre-alpha**

This crate builds on `nightly`. This will change in version `1.0.0`.

## Algorithms

### Single-source shortest paths

- `dijkstra_sssp_unweighted` for unweighted directed graphs
- `dijkstra_sssp_weighted` for weighted directed graphs

### Multiple sources shortest paths

- `Unweighted.dijkstra` for unweighted directed graphs
- `Weighted.dijkstra` for weighted directed graphs

## Graph operation traits

These traits are implemented for various graph representations built from standard library containers.

- `AddEdge` adds an edge to an unweighted graph.
- `AddWeightedEdge` adds an edge to a weighted graph.
- `CountAllEdges` counts all edges in a graph.
- `CountAllVertices` counts all vertices in a graph.
- `EdgeWeight` gets the weight of a given edge.
- `Indegree` returns the indegree of a given vertex.
- `IsEdge` returns whether an edge exists between two vertices.
- `IterAllEdges` iterates over all unweighted edges in a graph.
- `IterAllWeightedEdges` iterates over all weighted edges in a graph.
- `IterEdges` iterates over all unweighted edges with a given source vertex.
- `IterVertices` iterates over all vertices in a graph.
- `IterWeightedEdges` iterates over all weighted edges with a given source vertex.
- `Outdegree` returns the outdegree of a vertex.
- `RemoveEdge` removes an edge from a graph.
- `VertexWeight` returns the weight of a given vertex.

## Graph representations

### Adjacency list, unweighted

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
