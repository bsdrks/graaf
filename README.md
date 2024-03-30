# Graaf

Functions and types for working with graphs.

**WARNING: this crate is pre-alpha**

## Graph representations

### Adjacency list

- `Vec<Vec<usize>>`
- `Vec<Vec<(usize, usize)>>`
- `Vec<HashSet<usize>>`
- `Vec<HashSet<usize, W>>`
- `Vec<HashMap<usize, W>>`
- `[Vec<usize>]`
- `[Vec<(usize, usize)>]`
- `[HashSet<usize>]`
- `[HashSet<usize, W>]`
- `[HashMap<usize, W>]`
- `HashMap<usize, Vec<usize>>`
- `HashMap<usize, Vec<(usize, W)>>`
- `HashMap<usize, HashSet<usize>>`
- `HashMap<usize, HashSet<(usize, W)>>`
- `HashMap<usize, HashMap<usize, W>>`

### Adjacency matrix

- `AdjacencyMatrix`

### Edge list

- `Vec<(usize, usize)>`
- `Vec<(usize, usize, W)>`
- `[(usize, usize)]`
- `[(usize, usize, W)]`
- `HashSet<(usize, usize)>`
- `HashSet<(usize, usize, W)>`

## Graph operation traits

- `AddEdge` adds an edge to an unweighted graph
- `AddWeightedEdge` adds an edge to a weighted graph
- `CountAllEdges` counts all the edges in a graph
- `CountAllVertices` counts all the vertices in a graph
- `EdgeWeight` returns the weight of an edge
- `Indegree` returns the indegree of an edge
- `IsEdge` returns whether an edge exists in a graph
- `IterAllEdges` iterates over all the edges in an unweighted graph
- `IterAllWeightedEdges` iterates over all the edges in a weighted graph
- `IterEdges` iterates over all the edges from a vertex in an unweighted graph
- `IterVertices` iterates over all the vertices in a graph
- `IterWeightedEdges` iterates over all the edges from a vertex in a weighted graph
- `Outdegree` returns the outdegree of an edge
- `RemoveEdge` removes an edge from a graph
- `VertexWeight` returns the weight of a vertex

## Algorithms

### Shortest path

`DijkstraUnweighted`

Dijkstra's algorithm with binary heap for unweighted directed graphs. Works on graph representations that implement `AddEdge`.

`DijkstraWeighted`

Dijkstra's algorithm with binary heap for weighted directed graphs. Works on graph representations that implement `AddWeightedEdge`.
