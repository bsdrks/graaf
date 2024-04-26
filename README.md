# ![Graaf!](/logo.png "Graaf") &emsp; [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

[Graph algorithms](https://docs.rs/graaf/latest/graaf/algo/index.html), [operations](https://docs.rs/graaf/latest/graaf/op/index.html), [generators](https://docs.rs/graaf/latest/graaf/gen/index.html), and [representations](https://docs.rs/graaf/latest/graaf/repr/index.html).

See the [changelog](https://github.com/bsdrks/graaf/blob/main/CHANGELOG.md#provisional-roadmap) for a provisional roadmap.

1. [Installation](#installation)
2. [Usage](#usage)
3. [Features](#features)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
graaf = "0.26.1"
```

## Usage

```rust
use {
    graaf::{
        algo::bfs::single_pair_shortest_path,
        op::{
            AddEdge,
            Indegree,
        },
    },
    std::collections::HashSet,
};

let mut graph = [
    HashSet::new(), 
    HashSet::new(), 
    HashSet::new(), 
    HashSet::new()
];

// ╭───╮       ╭───╮
// │ 0 │   →   │ 1 │
// ╰───╯       ╰───╯
//   ↑           ↓
// ╭───╮       ╭───╮
// │ 3 │       │ 2 │
// ╰───╯       ╰───╯

graph.add_edge(3, 0);
graph.add_edge(0, 1);
graph.add_edge(1, 2);

assert_eq!(graph.indegree(0), 1);
assert_eq!(graph.indegree(1), 1);
assert_eq!(graph.indegree(2), 1);
assert_eq!(graph.indegree(3), 0);

let path = single_pair_shortest_path(&graph, 3, 2);

assert_eq!(path, Some(vec![3, 0, 1, 2]));
```

## Features

`adjacency_matrix`

This feature enables [`AdjacencyMatrix`](https://docs.rs/graaf/latest/graaf/repr/adjacency_matrix/struct.AdjacencyMatrix.html), a representation for dense graphs. The implementation requires nightly Rust. To opt out, change the `[dependencies]` entry for `graaf` in your `Cargo.toml`:

```toml
[dependencies]
graaf = { version = "0.26.1", default-features = false }
```
