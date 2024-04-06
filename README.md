# Graaf &emsp; [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Functions and types for working with graphs

**Graaf** is Dutch for

1. graph
2. count
3. dig

**This crate is in early alpha. The API is unstable.**

## Algorithms

### Breadth-first search

- `algo::bfs::min_distances_single_source`
- `algo::bfs::min_distances`
- `algo::bfs::predecessors_single_source`
- `algo::bfs::predecessors`

### Dijkstra's algorithm

- `algo::dijkstra::min_distances_single_source`
- `algo::dijkstra::min_distances`
- `algo::dijkstra::predecessors_single_source`
- `algo::dijkstra::predecessors`

## Graph operation traits

These traits are implemented for various graph representations built from standard library containers.

- `op::AddEdge` adds an unweighted edge.
- `op::AddWeightedEdge` adds a weighted edge.
- `op::CountAllEdges` counts all edges.
- `op::CountAllVertices` counts all vertices.
- `op::EdgeWeight` gets the weight of an edge.
- `op::Indegree` returns the indegree of a vertex.
- `op::IsEdge` returns whether an edge exists.
- `op::IterAllEdges` iterates over all unweighted edges.
- `op::IterAllWeightedEdges` iterates over all weighted edges.
- `op::IterEdges` iterates over all unweighted edges of a source vertex.
- `op::IterVertices` iterates over all vertices.
- `op::IterWeightedEdges` iterates over all weighted edges of a source vertex.
- `op::Outdegree` returns the outdegree of a vertex.
- `op::RemoveEdge` removes an edge.

## Graph representations

### Adjacency list

#### Unweighted

- `Vec<Vec<usize>>`
- `Vec<HashSet<usize>>`
- `[Vec<usize>]`
- `[HashSet<usize>]`
- `[Vec<usize>; V]`
- `[HashSet<usize>; V]`
- `HashMap<usize, Vec<usize>>`
- `HashMap<usize, HashSet<usize>>`

#### Weighted

- `Vec<Vec<(usize, W)>>`
- `Vec<HashSet<(usize, W)>>`
- `Vec<HashMap<usize, W>>`
- `[Vec<(usize, W)>]`
- `[HashSet<(usize, W)>]`
- `[HashMap<usize, W>]`
- `[Vec<(usize, W)>; V]`
- `[HashSet<(usize, W)>; V]`
- `[HashMap<usize, W>; V]`
- `HashMap<usize, Vec<(usize, W)>>`
- `HashMap<usize, HashSet<(usize, W)>>`
- `HashMap<usize, HashMap<usize, W>>`

### Adjacency matrix

- `AdjacencyMatrix`: an adjacency matrix representation of an unweighted directed graph stored as a bit array.

### Edge list

#### Unweighted

- `Vec<(usize, usize)>`
- `[(usize, usize)]`
- `[(usize, usize); V]`
- `HashSet<(usize, usize)>`

#### Weighted

- `Vec<(usize, usize, W)>`
- `[(usize, usize, W)]`
- `[(usize, usize, W); V]`
- `HashSet<(usize, usize, W)>`
