# Graaf

This crate contains functions and types for working with graphs.

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

## Graph operations

- Add edge
- Add weighted edge
- Count all edges
- Count all vertices
- Edge weight
- Indegree
- Is edge
- Iter all edges
- Iter all weighted edges
- Iter edges
- Iter vertices
- Iter weighted edges
- Outdegree
- Remove edge
- Vertex weight

## Algorithms

- Dijkstra's algorithm for unweighted directed graphs
- Dijkstra's algorithm for weighted directed graphs
