# Changelog

## [0.30.2] - 2024-04-29

Changed

- Add more extensive examples to `README`.
- Simplify `strategy::binop_vertices` implementation.

## [0.30.1] - 2024-04-28

Fixed

- Fix `prop::strategy::binop_vertices` when `v` is zero inside `flat_map`.

## [0.30.0] - 2024-04-28

Removed

- Breaking: Remove `prop::remove_edge_is_edge`.

## [0.29.1] - 2024-04-28

Added

- Add `add_weighted_edge_is_edge` property test `btree_map_btree_set`.
- Add `add_weighted_edge_is_edge` property test `hash_map_hash_set`.
- Add `add_weighted_edge_is_edge` property test `slice_btree_set`.
- Add `add_weighted_edge_is_edge` property test `slice_hash_set`.
- Add `add_weighted_edge_is_edge` property test `vec_btree_map`.
- Add `add_weighted_edge_is_edge` property test `vec_hash_map`.
- Add `add_weighted_edge_is_edge` unit test `arr_btree_map`.
- Add `add_weighted_edge_is_edge` unit test `arr_hash_map`.
- Add `add_weighted_edge_remove_edge` property test `btree_map_btree_map`.
- Add `add_weighted_edge_remove_edge` property test `hash_map_hash_map`.
- Add `add_weighted_edge_remove_edge` property test `slice_btree_map`.
- Add `add_weighted_edge_remove_edge` property test `slice_hash_map`.
- Add `add_weighted_edge_remove_edge` property test `vec_btree_map`.
- Add `add_weighted_edge_remove_edge` property test `vec_hash_map`.
- Add `add_weighted_edge_remove_edge` unit test `arr_btree_map`.
- Add `add_weighted_edge_remove_edge` unit test `arr_hash_map`.

Changed

- Loosen bound on `G` in `add_weighted_edge_is_edge` to `?Sized`.

Fixed

- Fix `prop::strategy::binop_vertices` when `v` is zero.

Removed

- Remove `add_weighted_edge_is_edge` unit test `vec_hash_map`.
- Remove `add_weighted_edge_remove_edge` unit test `vec_hash_map`.
- Rename `prop::strategy::v_s_t` to `prop::strategy::binop_vertices`.

## [0.29.0] - 2024-04-28

Added

- Add `add_edge_is_edge` property test `btree_map_btree_set`.
- Add `add_edge_is_edge` property test `hash_map_hash_set`.
- Add `add_edge_is_edge` property test `slice_btree_set`.
- Add `add_edge_is_edge` property test `slice_hash_set`.
- Add `add_edge_is_edge` property test `vec_btree_set`.
- Add `add_edge_is_edge` property test `vec_hash_set`.
- Add `add_edge_is_edge` unit test `arr_btree_set`.
- Add `add_edge_remove_edge` property test `vec_btree_set`.
- Add `add_edge_remove_edge` property test `vec_hash_set`.
- Add `add_edge_remove_edge` unit test `arr_btree_set`.
- Add `prop::strategy::v_s_t` to order, source, and target.
- Add `proptest` as a dev-dependency.

Changed

- Breaking: Move `op::prop` to `prop`.

Removed

- Remove `add_edge_is_edge` unit test `hash_map_hash_set`.
- Remove `add_edge_is_edge` unit test `hash_map_hash_set`.
- Remove `add_edge_is_edge` unit test `vec_hash_set`.
- Remove `add_edge_remove_edge` unit test `vec_hash_set`.

## [0.28.2] - 2024-04-28

Added

- Add `gen::LinearConst` generator.
- Implement `LinearConst` for `[BTreeSet<usize>; V]`.
- Implement `LinearConst` for `[HashSet<usize>; V]`.
- Implement `LinearConst` for `[Vec<usize>; V]`.
- Implement `Linear` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Linear` for `BTreeMap<usize, Vec<usize>>`.

Fixed

- Fix comments in `gen::Linear` test function bodies.

## [0.28.1] - 2024-04-27

Added

- Add `gen::Cycle` generator.

## [0.28.0] - 2024-04-27

Added

- Add `algo::dijkstra::shortest_path`.
- Add `bench` job to GHA workflow.
- Bench `add_edge` for `BTreeMap<usize, BTreeSet<usize>>`.
- Bench `add_edge` for `BTreeMap<usize, Vec<usize>>`.
- Bench `add_edge` for `Vec<BTreeSet<usize>>`.
- Bench `add_edge` for `[BTreeSet<usize>; V]`.
- Bench `count_all_edges` for `Vec<BTreeMap<usize, usize>>`.

Changed

- Breaking: Return `bfs::shortest_path` when target is found before pushing the target to the queue.
- Compress textual diagrams.
- Cross-link `bfs` and `dijkstra` in module documentation.
- Document reasons for panic in `bfs` and `dijkstra`.
- Link to `op` in the `repr` module documentation.
- Remove `min_time` from benches.

## [0.27.1] - 2024-04-26

Changed

- Check top changelog entry version number on pre-commit.

## [0.27.0] - 2024-04-26

Added

- Add `op::Target`, a trait to get the target vertex of an adjacency list edge.

Changed

- Replace `Linear for G: AddEdge + Empty` with `Linear for Vec<BTreeSet<usize>>`.
- Replace `Linear for G: AddEdge + Empty` with `Linear for Vec<HashSet<usize>>`.
- Replace `Linear for G: AddEdge + Empty` with `Linear for Vec<Vec<usize>>`.

Removed

- Breaking: Remove `gen::Empty`.

## [0.26.1] - 2024-04-26

Added

- Add `gen::Empty`, a generator for empty graphs.
- Implement `Linear` for `G: AddEdge + Empty`.
- Implement `Empty` for `Vec<T: Clone + Default + IntoIterator<Item = usize>>`.

Changed

- Replace `Linear for Vec<BTreeSet<usize>>` with `Linear for G: AddEdge + Empty`.
- Replace `Linear for Vec<HashSet<usize>>` with `Linear for G: AddEdge + Empty`.
- Replace `Linear for Vec<Vec<usize>>` with `Linear for G: AddEdge + Empty`.

## [0.26.0] - 2024-04-25

Changed

- Breaking: Remove `nightly` feature, as it is only needed for `adjacency_matrix`.
- Simplify `README`.

## [0.25.2] - 2024-04-25

Changed

- Split up GHA workflow into multiple jobs.

## [0.25.1] - 2024-04-25

Changed

- Replace `saturating_sub` with `-` in `gen::linear`.

## [0.25.0] - 2024-04-24

Added

- Implement `Linear` for `BTreeSet<usize>`.
- Implement `Linear` for `HashSet<usize>`.

Fixed

- Fix `Linear::linear`.

## [0.24.1] - 2024-04-24

Added

- Add doctest for `gen::Linear`.

## [0.24.0] - 2024-04-23

Added

- Add `gen::Linear`, a generator for linear graphs.

Changed

- `IsEdge` for `[BTreeMap<usize, W>]` no longer panics if `s` is not in the graph.
- `IsEdge` for `[BTreeSet<usize>]` no longer panics if `s` is not in the graph.
- `IsEdge` for `[HashMap<usize, W>]` no longer panics if `s` is not in the graph.
- `IsEdge` for `[HashSet<usize>]` no longer panics if `s` is not in the graph.

## [0.23.1] - 2024-04-23

Added

- Add more tests for `bfs::single_pair_shortest_path`.
- Mention `bfs::single_pair_shortest_path` in the module documentation of `bfs`.

## [0.23.0] - 2024-04-22

Added

- Add `bfs::single_pair_shortest_path`.

Changed

- Breaking: remove `?Sized` bound from `graph` in `bfs::distances`.
- Breaking: remove `?Sized` bound from `graph` in `bfs::predecessors`.
- Change `is_target` to `impl Fn(W) -> bool` in `bfs::shortest_path`.
- Change `step` type to `impl Fn(W) -> W` in `bfs::distances`.
- Change `step` type to `impl Fn(W) -> W` in `bfs::predecessors`.
- Change `step` type to `impl Fn(W) -> W` in `bfs::shortest_path`.
- Change `step` type to `impl Fn(W) -> W` in `dijkstra::distances`.
- Change `step` type to `impl Fn(W) -> W` in `dijkstra::predecessors`.
- Update `README.md` example to use `bfs::single_pair_shortest_path`.

## [0.22.1] - 2024-04-21

Changed

- Move module documentation into modules.
- Use `HashSet` instead of `AdjacencyMatrix` in `lib` test.
- Add `Cargo.toml` to `.gitignore`.

## [0.22.0] - 2024-04-21

Changed

- Breaking: Disable `adjacency_matrix` by default to support stable Rust.

## [0.21.0] - 2024-04-21

Added

- Add `nightly` feature.

Changed

- Breaking: Make the `adjacency_matrix` depend on the `nightly` feature.
- Breaking: Enable `adjacency_matrix` by default.
- Replace `assert_matches!(...)` with `assert!(matches!(...))`.

## [0.20.2] - 2024-04-21

Added

- Add `pre-commit` configuration.

## [0.20.1] - 2024-04-21

Changed

- Derive `Hash` for `AdjacencyMatrix`.
- Derive `Ord` for `AdjacencyMatrix`.

## [0.20.0] - 2024-04-20

Added

- Add `predecessor::search_by`.
- Add `bfs::shortest_path`.

Changed

- Breaking: Rename `bfs::distances_single_source` to `bfs::single_source_distances`.
- Breaking: Rename `bfs::predecessors_single_source` to `bfs::single_source_predecessors`.
- Breaking: Rename `dijkstra::distances_single_source` to `dijkstra::single_source_distances`.
- Breaking: Rename `dijkstra::predecessors_single_source` to `dijkstra::single_source_predecessors`.
- Breaking: `predecessor::search` now returns a singleton path if the target is the source.

## [0.19.0] - 2024-04-20

Changed

- Breaking: `bfs::predecessors_single_source` now only returns the predecessor tree.
- Breaking: `dijkstra::predecessors_single_source` now only returns the predecessor tree.

## [0.18.0] - 2024-04-20

Changed

- Breaking: `repr::AdjacenyMatrix` is now a feature.

## [0.17.2] - 2024-04-19

Added

- Add `predecessor::search`.

## [0.17.1] - 2024-04-19

Changed

- Move list of standard library graph respresentations from `README` to `op/mod.rs`.

## [0.17.0] - 2024-04-18

Added

- Implement `Indegree` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `Indegree` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Indegree` for `Vec<BTreeMap<usize, W>>`.
- Implement `Indegree` for `Vec<BTreeSet<usize>>`.
- Implement `Indegree` for `[BTreeMap<usize, W>; V]`.
- Implement `Indegree` for `[BTreeMap<usize, W>]`.
- Implement `Indegree` for `[BTreeSet<usize>; V]`.
- Implement `Indegree` for `[BTreeSet<usize>]`.
- Implement `IterVertices` for `Vec<BTreeMap<usize, W>>`.
- Implement `IterVertices` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `IterVertices` for `Vec<BTreeSet<usize>>`.
- Implement `IterVertices` for `Vec<HashMap<usize, W>>`.
- Implement `IterVertices` for `Vec<HashSet<(usize, W)>>`.
- Implement `IterVertices` for `Vec<HashSet<usize>>`.
- Implement `IterVertices` for `Vec<Vec<(usize, W)>>`.
- Implement `IterVertices` for `Vec<Vec<usize>>`.
- Implement `IterVertices` for `[BTreeMap<usize, W>; V]`.
- Implement `IterVertices` for `[BTreeMap<usize, W>]`.
- Implement `IterVertices` for `[BTreeSet<(usize, W); V]`.
- Implement `IterVertices` for `[BTreeSet<(usize, W)]`.
- Implement `IterVertices` for `[BTreeSet<usize>; V]`.
- Implement `IterVertices` for `[BTreeSet<usize>]`.
- Implement `IterVertices` for `[HashMap<usize, W>; V]`.
- Implement `IterVertices` for `[HashMap<usize, W>]`.
- Implement `IterVertices` for `[HashSet<(usize, W); V]`.
- Implement `IterVertices` for `[HashSet<(usize, W)]`.
- Implement `IterVertices` for `[HashSet<usize>; V]`.
- Implement `IterVertices` for `[HashSet<usize>]`.
- Implement `IterVertices` for `[Vec<(usize, W)>; V]`.
- Implement `IterVertices` for `[Vec<(usize, W)>]`.
- Implement `IterVertices` for `[Vec<usize>; V]`.
- Implement `IterVertices` for `[Vec<usize>]`.

Removed

- Breaking: Remove `IterVertices` for `Vec<T>`.
- Breaking: Remove `IterVertices` for `[T]`.
- Breaking: Remove `IterVertices` for `[T; V]`.

Fixed

- Remove unused parameter `W` from `remove_edge_is_edge`.

## [0.16.1] - 2024-04-17

Added

- Implement `RemoveEdge` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `RemoveEdge` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `RemoveEdge` for `Vec<BTreeMap<usize, W>>`.
- Implement `RemoveEdge` for `Vec<BTreeSet<usize>>`.
- Implement `RemoveEdge` for `[BTreeMap<usize, W>; V]`.
- Implement `RemoveEdge` for `[BTreeMap<usize, W>]`.
- Implement `RemoveEdge` for `[BTreeSet<usize>; V]`.
- Implement `RemoveEdge` for `[BTreeSet<usize>]`.

Changed

- `RemoveEdge::remove_edge` now returns a `bool` indicating whether the edge was removed.

## [0.16.0] - 2024-04-17

Added

- Implement `Outdegree` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `Outdegree` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Outdegree` for `BTreeMap<usize, Vec<usize>>`.
- Implement `Outdegree` for `Vec<BTreeMap<usize, W>>`.
- Implement `Outdegree` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `Outdegree` for `Vec<BTreeSet<usize>>`.
- Implement `Outdegree` for `Vec<HashMap<usize, W>>`.
- Implement `Outdegree` for `Vec<HashSet<(usize, W)>>`.
- Implement `Outdegree` for `Vec<HashSet<usize>>`.
- Implement `Outdegree` for `Vec<Vec<(usize, W)>>`.
- Implement `Outdegree` for `Vec<Vec<usize>>`.
- Implement `Outdegree` for `[BTreeMap<usize, W>; V]`.
- Implement `Outdegree` for `[BTreeMap<usize, W>]`.
- Implement `Outdegree` for `[BTreeSet<(usize, W)>; V]`.
- Implement `Outdegree` for `[BTreeSet<(usize, W)>]`.
- Implement `Outdegree` for `[BTreeSet<usize>; V]`.
- Implement `Outdegree` for `[BTreeSet<usize>]`.
- Implement `Outdegree` for `[HashMap<usize, W>; V]`.
- Implement `Outdegree` for `[HashMap<usize, W>]`.
- Implement `Outdegree` for `[HashSet<(usize, W)>; V]`.
- Implement `Outdegree` for `[HashSet<(usize, W)>]`.
- Implement `Outdegree` for `[HashSet<usize>]; V`.
- Implement `Outdegree` for `[HashSet<usize>]`.
- Implement `Outdegree` for `[Vec<(usize, W)>; V]`.
- Implement `Outdegree` for `[Vec<(usize, W)>]`.
- Implement `Outdegree` for `[Vec<usize>; V]`.
- Implement `Outdegree` for `[Vec<usize>]`.

Changed

- Breaking: Remove `Outdegree` for `Vec<HashMap<K, W>>`.
- Breaking: Remove `Outdegree` for `Vec<HashSet<T>>`.
- Breaking: Remove `Outdegree` for `Vec<Vec<T>>`.
- Breaking: Remove `Outdegree` for `[HashMap<K, W>; V]`.
- Breaking: Remove `Outdegree` for `[HashMap<K, W>]`.
- Breaking: Remove `Outdegree` for `[HashSet<T>; V]`.
- Breaking: Remove `Outdegree` for `[HashSet<T>]`.
- Breaking: Remove `Outdegree` for `[Vec<T>; V]`.
- Breaking: Remove `Outdegree` for `[Vec<T>]`.

## [0.15.1] - 2024-04-17

Added

- Implement `IterWeightedEdges` for `Vec<BTreeMap<usize, W>>`.
- Implement `IterWeightedEdges` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `IterWeightedEdges` for `[BTreeMap<usize, W>; V]`.
- Implement `IterWeightedEdges` for `[BTreeMap<usize, W>]`.
- Implement `IterWeightedEdges` for `[BTreeSet<(usize, W)>; V]`.
- Implement `IterWeightedEdges` for `[BTreeSet<(usize, W)>]`.
- Implement `IterWeightedEdges` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `IterWeightedEdges` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `IterWeightedEdges` for `BTreeMap<usize, BTreeMap<usize, W>>`.

## [0.15.0] - 2024-04-16

Removed

- Breaking: Remove `IterVertices` for `HashMap<_>`.

## [0.14.2] - 2024-04-16

Added

- Implement `IterEdges` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IterEdges` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterEdges` for `Vec<BTreeSet<usize>>`.
- Implement `IterEdges` for `[BTreeSet<usize>; V]`.
- Implement `IterEdges` for `[BTreeSet<usize>]`.

Fixes

- Fix benchmark imports.

## [0.14.1] - 2024-04-16

Added

- Implement `IsSimple` for `Vec<BTreeSet<usize>>`.
- Implement `IsSimple` for `[BTreeSet<usize>]`.
- Implement `IsSimple` for `[BTreeSet<usize>; V]`.
- Implement `IsSimple` for `BTreeSet<(usize, usize)>`.
- Implement `IsSimple` for `BTreeSet<(usize, usize, W)>`.
- Implement `IterAllEdges` for `BTreeSet<(usize, usize)>`.
- Implement `IterAllWeightedEdges` for `BTreeSet<(usize, usize, W)>`.

## [0.14.0] - 2024-04-15

Changed

- Breaking: Rename `bfs::min_distances` to `bfs::distances`.
- Breaking: Rename `bfs::min_distances_single_source` to `bfs::distances_single_source`.
- Breaking: Rename `dijkstra::min_distances` to `dijkstra::distances`.
- Breaking: Rename `dijkstra::min_distances_single_source` to `dijkstra::distances_single_source`.

## [0.13.3] - 2024-04-15

Added

- Add logo to `README`.

## [0.13.2] - 2024-04-15

Added

- Implement `CountAllVertices` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `CountAllVertices` for `Vec<HashSet<(usize, W)>>`.
- Implement `CountAllVertices` for `[BTreeSet<(usize, W)>; V]`.
- Implement `CountAllVertices` for `[BTreeSet<(usize, W)>]`.
- Implement `CountAllVertices` for `[HashSet<(usize, W); V]`.
- Implement `CountAllVertices` for `[HashSet<(usize, W)]`.

Fixed

- Fix `cross_country` benchmark.
- Fix `shortest_path_1` benchmark.
- Fix `small_graph_1` benchmark.

## [0.13.1] - 2024-04-15

Added

- Implement `IsEdge` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsEdge` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsEdge` for `BTreeSet<(usize, usize)>`.
- Implement `IsEdge` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsEdge` for `Vec<BTreeSet<usize>>`.
- Implement `IsEdge` for `[BTreeMap<usize, W>; V]`.
- Implement `IsEdge` for `[BTreeMap<usize, W>]`.
- Implement `IsEdge` for `[BTreeSet<usize>; V]`.
- Implement `IsEdge` for `[BTreeSet<usize>]`.

## [0.13.0] - 2024-04-14

Added

- Implement `CountAllVertices` for `Vec<BTreeMap<usize, W>>`.
- Implement `CountAllVertices` for `Vec<BTreeSet<usize>>`.
- Implement `CountAllVertices` for `Vec<HashMap<usize, W>>`.
- Implement `CountAllVertices` for `Vec<HashSet<usize>>`.
- Implement `CountAllVertices` for `Vec<Vec<(usize, W)>>`.
- Implement `CountAllVertices` for `Vec<Vec<usize>>`.
- Implement `CountAllVertices` for `[BTreeMap<usize, W>; V]`.
- Implement `CountAllVertices` for `[BTreeMap<usize, W>]`.
- Implement `CountAllVertices` for `[BTreeSet<usize>; V]`.
- Implement `CountAllVertices` for `[BTreeSet<usize>]`.
- Implement `CountAllVertices` for `[HashMap<usize, W>; V]`.
- Implement `CountAllVertices` for `[HashMap<usize, W>]`.
- Implement `CountAllVertices` for `[HashSet<usize>; V]`.
- Implement `CountAllVertices` for `[HashSet<usize>]`.
- Implement `CountAllVertices` for `[Vec<(usize, W)>; V]`.
- Implement `CountAllVertices` for `[Vec<(usize, W)>]`.
- Implement `CountAllVertices` for `[Vec<usize>; V]`.
- Implement `CountAllVertices` for `[Vec<usize>]`.

Removed

- Breaking: Remove `CountAllVertices` for `HashMap<_>`.
- Breaking: Remove `CountAllVertices` for `Vec<T>`.
- Breaking: Remove `CountAllVertices` for `[T; V]`.
- Breaking: Remove `CountAllVertices` for `[T]`.

## [0.12.1] - 2024-04-14

Added

- Implement `CountAllEdges` for `BTreeMap<K, BTreeMap<K, W>>`.
- Implement `CountAllEdges` for `BTreeMap<K, BTreeSet<T>>`.
- Implement `CountAllEdges` for `BTreeMap<K, Vec<T>>`.
- Implement `CountAllEdges` for `Vec<BTreeMap<K, W>>`.
- Implement `CountAllEdges` for `Vec<BTreeSet<T>>`.
- Implement `CountAllEdges` for `[BTreeMap<K, W>; V]`.
- Implement `CountAllEdges` for `[BTreeMap<K, W>]`.
- Implement `CountAllEdges` for `[BTreeSet<T>; V]`.
- Implement `CountAllEdges` for `[BTreeSet<T>]`.
- Implement `IsEdge` for `Vec<HashMap<usize, W>>`.
- Implement `IsEdge` for `[HashMap<usize, W>; V]`.

Removed

- Remove `Ord` bound from trait implementations where possible.

## [0.12.0] - 2024-04-14

Added

- Implement `AddWeightedEdge` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `AddWeightedEdge` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `AddWeightedEdge` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `AddWeightedEdge` for `Vec<BTreeMap<usize, W>>`.
- Implement `AddWeightedEdge` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `AddWeightedEdge` for `[BTreeMap<usize, W>; V]`.
- Implement `AddWeightedEdge` for `[BTreeMap<usize, W>]`.
- Implement `AddWeightedEdge` for `[BTreeSet<(usize, W)>; V]`.
- Implement `AddWeightedEdge` for `[BTreeSet<(usize, W)>]`.

Changed

- Breaking: `AddWeightedEdge` for `HashMap<_>` now panics if `s` is not in the graph.

## [0.11.1] - 2024-04-14

Added

- Implement `AddEdge` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `AddEdge` for `BTreeMap<usize, Vec<usize>>`.
- Implement `AddEdge` for `Vec<BTreeSet<usize>>`.
- Implement `AddEdge` for `[BTreeSet<usize>; V]`.
- Implement `AddEdge` for `[BTreeSet<usize>]`.

## [0.11.0] - 2024-04-14

Added back `op` implementations for `Vec` and arrays to simplify use cases.

Added

- Implement `AddEdge` for `Vec<HashSet<usize>>`.
- Implement `AddEdge` for `Vec<Vec<usizee>>`.
- Implement `AddEdge` for `[HashSet<usize>; V]`.
- Implement `AddEdge` for `[Vec<usize>; V]`.
- Implement `AddWeightedEdge` for `Vec<HashMap<usize, W>>`.
- Implement `AddWeightedEdge` for `Vec<HashSet<(usize, W)>>`.
- Implement `AddWeightedEdge` for `Vec<Vec<(usize, W)>>`.
- Implement `AddWeightedEdge` for `[HashMap<usize, W>; V]`.
- Implement `AddWeightedEdge` for `[HashSet<(usize, W)>; V]`.
- Implement `AddWeightedEdge` for `[Vec<(usize, W)>; V]`.
- Implement `CountAllEdges` for `Vec<HashMap<K, W>>`.
- Implement `CountAllEdges` for `Vec<HashSet<T>>`.
- Implement `CountAllEdges` for `Vec<Vec<T>>`.
- Implement `CountAllEdges` for `[HashMap<K, W>; V]`.
- Implement `CountAllEdges` for `[HashSet<T>; V]`.
- Implement `CountAllEdges` for `[Vec<T>; V]`.
- Implement `EdgeWeight` for `Vec<HashMap<usize, W>>`.
- Implement `EdgeWeight` for `[HashMap<usize, W>; V]`.
- Implement `Indegree` for `Vec<HashMap<usize, W>>`.
- Implement `Indegree` for `Vec<HashSet<usize>>`.
- Implement `Indegree` for `[HashMap<usize, W>; V]`.
- Implement `Indegree` for `[HashSet<usize>; V]`.
- Implement `IsEdge` for `Vec<HashSet<usize>>`.
- Implement `IsEdge` for `[HashMap<usize, W>; V]`.
- Implement `IsSimple` for `Vec<(usize, usize)>`.
- Implement `IsSimple` for `Vec<(usize, usize, W)>`.
- Implement `IsSimple` for `Vec<HashSet<usize>>`.
- Implement `IsSimple` for `[(usize, usize); V]`.
- Implement `IsSimple` for `[(usize, usize, W); V]`.
- Implement `IsSimple` for `[HashSet<usize>; V]`.
- Implement `IterAllEdges` for `Vec<(usize, usize)>`.
- Implement `IterAllEdges` for `[(usize, usize); V]`.
- Implement `IterAllWeightedEdges` for `Vec<(usize, usize, W)>`.
- Implement `IterAllWeightedEdges` for `[(usize, usize, W); V]`.
- Implement `IterVertices` for `Vec<T>`.
- Implement `IterVertices` for `[T; V]`.
- Implement `Outdegree` for `Vec<HashMap<K, W>>`.
- Implement `Outdegree` for `Vec<HashSet<T>>`.
- Implement `Outdegree` for `Vec<Vec<T>>`.
- Implement `Outdegree` for `[HashMap<K, W>; V]`.
- Implement `Outdegree` for `[HashSet<T>; V]`.
- Implement `Outdegree` for `[Vec<T>; V]`.
- Implement `RemoveEdge` for `Vec<HashMap<usize, W>>`.
- Implement `RemoveEdge` for `Vec<HashSet<usize>>`.
- Implement `RemoveEdge` for `[HashMap<usize, W>; V]`.
- Implement `RemoveEdge` for `[HashSet<usize>; V]`.
- Test `add_edge_remove_edge` for `AdjacencyMatrix`.
- Test `add_edge_remove_edge` for `Vec<HashSet<usize>>`.
- Test `add_edge_remove_edge` for `[HashSet<usize>; V]`.

Fixed

- Remove stray `W` type parameter in `add_edge_remove_edge`.

## [0.10.0] - 2024-04-12

Added

- Implement `IsSimple` for `AdjacencyMatrix`.

Fixed

- `IsSimple` now checks for parallel edges in `HashSet<(usize, usize, W)>`.

## [0.9.0] - 2024-04-12

Fixed

- `IsSimple` now checks for parallel edges in `HashSet<(usize, usize)>`.
- `IsSimple` now checks for parallel edges in `[(usize, usize)]`.
- `IsSimple` now checks for parallel edges in `[(usize, usize, W)]`.

## [0.8.4] - 2024-04-11

Fixed

- Add missing property `add_weighted_edge_remove_edge`.

## [0.8.3] - 2024-04-11

Added

- Test `add_edge_is_edge` for implementors of `AddEdge` and `IsEdge`.

## [0.8.2] - 2024-04-10

Added

- Implement `IsSimple` for `HashSet<(usize, usize)>`.
- Implement `IsSimple` for `HashSet<(usize, usize, W)>`.
- Implement `IsSimple` for `[(usize, usize)]`.
- Implement `IsSimple` for `[(usize, usize, W)]`.
- Implement `IsSimple` for `[HashSet<usize>]`.

## [0.8.1] - 2024-04-10

Added

- Add `IsSimple` trait.
- Implement `IsSimple` for `[HashSet<usize>]`.

Removed

- Remove `EdgeWeight` for `Vec<HashMap<usize, W>>`.
- Remove `EdgeWeight` for `[HashMap<usize, W>; V]`.
- Remove `IterVertices` for `HashSet<T>`.

## [0.8.0] - 2024-04-09

Added

- Add installation instructions to `README`.
- Add example usage to `README`.

Removed

- Remove `AddEdge` for `Vec<HashSet<T>>`.
- Remove `AddEdge` for `Vec<Vec<T>>`.
- Remove `AddEdge` for `[HashSet<T>; V]`.
- Remove `AddEdge` for `[Vec<T>; V]`.
- Remove `AddWeightedEdge` for `Vec<HashMap<usize, W>>`.
- Remove `AddWeightedEdge` for `Vec<HashSet<(usize, W)>>`.
- Remove `AddWeightedEdge` for `Vec<Vec<(usize, W)>>`.
- Remove `AddWeightedEdge` for `[HashMap<usize, W>; V]`.
- Remove `AddWeightedEdge` for `[HashSet<(usize, W)>; V]`.
- Remove `AddWeightedEdge` for `[Vec<(usize, W)>; V]`.

Changed

- Change `iter_all_edges` returns type to `impl Iterator<Item = (usize, usize)>`.
- Change `iter_all_weighted_edges` return type to `impl Iterator<Item = (usize, usize, &W)>`.
- Change `iter_weighted_edges` return type to `impl Iterator<Item = (usize, &W)>`.

## [0.7.0] - 2024-04-07

Added

- Implement `CountAllEdges` for `[HashMap<K, W>]`.
- Implement `CountAllEdges` for `[HashSet<T>]`.
- Implement `CountAllEdges` for `[Vec<T>]`.
- Implement `CountAllVertices` for `[T]`.
- Implement `EdgeWeight` for `[HashMap<usize, W>]`.
- Implement `Indegree` for `[HashMap<usize, W>]`.
- Implement `Indegree` for `[HashSet<usize>]`.
- Implement `IsEdge` for `[HashMap<usize, W>]`.
- Implement `IsEdge` for `[HashSet<usize>]`.
- Implement `IterAllEdges` for `[(usize, usize)]`.
- Implement `IterAllWeightedEdges` for `[(usize, usize, W)]`.
- Implement `IterEdges` for `[HashSet<usize>]`.
- Implement `IterEdges` for `[Vec<usize>]`.
- Implement `IterVertices` for `&[T]`.
- Implement `IterWeightedEdges` for `[HashMap<usize, W>]`.
- Implement `IterWeightedEdges` for `[HashSet<(usize, W)>]`.
- Implement `IterWeightedEdges` for `[Vec<(usize, W)>]`.
- Implement `Outdegree` for `[HashMap<K, W>]`.
- Implement `Outdegree` for `[HashSet<T>]`.
- Implement `Outdegree` for `[Vec<T>]`.
- Implement `RemoveEdge` for `[HashMap<usize, W>]`.
- Implement `RemoveEdge` for `[HashSet<usize>]`.

Changed

- Return `(&'a usize, &'a W)` from `iter_weighted_edges`.

Removed

- Remove `CountAllEdges` for `Vec<HashMap<K, W>>`.
- Remove `CountAllEdges` for `Vec<HashSet<T>>`.
- Remove `CountAllEdges` for `Vec<Vec<T>>`.
- Remove `CountAllEdges` for `[HashMap<K, W>; V]`.
- Remove `CountAllEdges` for `[HashSet<T>; V]`.
- Remove `CountAllEdges` for `[Vec<T>; V]`.
- Remove `Indegree` for `Vec<HashMap<usize, W>>`.
- Remove `Indegree` for `Vec<HashSet<usize>>`.
- Remove `Indegree` for `[HashMap<usize, W>; V]`.
- Remove `Indegree` for `[HashSet<usize>; V]`.
- Remove `IsEdge` for `Vec<HashMap<usize, W>>`.
- Remove `IsEdge` for `Vec<HashSet<usize>>`.
- Remove `IsEdge` for `[HashMap<usize, W>; V]`.
- Remove `IsEdge` for `[HashSet<usize>; V]`.
- Remove `IterAllEdges` for `Vec<(usize, usize)>`.
- Remove `IterAllEdges` for `[(usize, usize); V]`.
- Remove `IterAllWeightedEdges` for `Vec<(usize, usize, W)>`.
- Remove `IterAllWeightedEdges` for `[(usize, usize, W); V]`.
- Remove `IterVertices` for `Vec<T>`.
- Remove `IterVertices` for `[T; V]`.
- Remove `Outdegree` for `Vec<HashMap<usize, W>>`.
- Remove `Outdegree` for `Vec<HashSet<usize>>`.
- Remove `Outdegree` for `Vec<Vec<T>>`.
- Remove `Outdegree` for `[HashMap<usize, W>; V]>`.
- Remove `Outdegree` for `[HashSet<usize>; V]>`.
- Remove `Outdegree` for `[Vec<T>; V]`.
- Remove `RemoveEdge` for `Vec<HashMap<usize, W>>`.
- Remove `RemoveEdge` for `Vec<HashSet<usize>>`.
- Remove `RemoveEdge` for `[HashMap<usize, W>; V]`.
- Remove `RemoveEdge` for `[HashSet<usize>; V]`.

## [0.6.3] - 2024-04-06

Fix

- Fix `README` formatting.

## [0.6.2] - 2024-04-06

Added

- Add more tests to `algo::bfs`.
- Add more tests to `algo::dijkstra`.
- Implement `AddEdge` for `[HashSet<usize>]`.
- Implement `AddEdge` for `[Vec<usize>]`.
- Implement `AddWeightedEdge` for `[HashMap<usize, W>]`.
- Implement `AddWeightedEdge` for `[HashSet<(usize, W)>]`.
- Implement `AddWeightedEdge` for `[Vec<(usize, W)>]`.
- Implement `IterAllEdges` for `[(usize, usize)]`.
- Implement `IterAllWeightedEdges` for `[(usize, usize, W)]`.

## [0.6.1] - 2024-04-06

Added

- Add "algorithms" and "mathematics" to `Cargo.toml` categories.
- Add "bfs" and "dijkstra" to `Cargo.toml` keywords.

Removed

- Remove redundant `homepage` metadata.

## [0.6.0] - 2024-04-06

Added

- Add `authors` to `README.md`.
- Add doctest for `AdjacencyMatrix::new`.
- Add doctest for `AdjacencyMatrix::toggle`.
- Add implementation documentation for `AddEdge`.
- Add implementation documentation for `AddWeightedEdge`.
- Add implementation documentation for `CountAllEdges`.
- Add implementation documentation for `CountAllVertices`.
- Add implementation documentation for `EdgeWeight`.
- Add implementation documentation for `Indegree`.
- Add implementation documentation for `IsEdge`.
- Add implementation documentation for `IterAllEdges`.
- Add implementation documentation for `IterAllWeightedEdges`.
- Add implementation documentation for `IterEdges`.
- Add implementation documentation for `IterWeightedEdges`.
- Add implementation documentation for `Outdegree`.
- Add implementation documentation for `RemoveEdge`.

Changed

- Move `ops` to `op`.
- Adapt benchmark code to linting rules.
- Move doctest trait properties to `op::prop`.
- Move lints from `lib.rs` to `Cargo.toml`.

## [0.5.3] - 2024-04-05

Added

- Add doctest for `op::add_weighted_edge::AddWeightedEdge`.
- Add doctest for `op::count_all_edges::CountAllEdges`.
- Add doctest for `op::count_all_vertices::CountAllVertices`.
- Add doctest for `op::edge_weight::EdgeWeight`.
- Add doctest for `op::indegree::Indegree`.
- Add doctest for `op::is_edge::IsEdge`.
- Add doctest for `op::iter_all_edges::IterAllEdges`.
- Add doctest for `op::iter_all_weighted_edges::IterAllWeightedEdges`.
- Add doctest for `op::iter_edges::IterEdges`.
- Add doctest for `op::iter_vertices::IterVertices`.
- Add doctest for `op::iter_weighted_edges::IterWeightedEdges`.
- Add doctest for `op::outdegree::OutDegree`.
- Add doctest for `op::remove_edge::RemoveEdge`.
- Add documentation for `op::add_weighted_edge`.
- Add documentation for `op::count_all_edges`.
- Add documentation for `op::count_all_vertices`.
- Add documentation for `op::edge_weight`.
- Add documentation for `op::indegree`.
- Add documentation for `op::is_edge`.
- Add documentation for `op::iter_all_edges`.
- Add documentation for `op::iter_all_weighted_edges`.
- Add documentation for `op::iter_edges`.
- Add documentation for `op::iter_vertices`.
- Add documentation for `op::iter_weighted_edges`.
- Add documentation for `op::outdegree`.
- Add documentation for `op::remove_edge`.

## [0.5.2] - 2024-04-04

Added

- Add doctest for `op::add_edge::AddEdge`.
- Add documentation for `op::add_edge`.
- Add module-level doctest for `algo::bfs`.
- Add module-level doctest for `algo::dijkstra`.

## [0.5.1] - 2024-04-04

Added

- Add `bfs::predecessors_single_source`.
- Add `dijkstra::predecessors_single_source`.

## [0.5.0] - 2024-04-03

Added

- Implement `Indegree` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsEdge` for `HashSet<(usize, usize)>`.
- Implement `IterVertices` for `HashSet<T>`.
- Implement `IterWeightedEdges` for `Vec<HashMap<usize, W>>`.
- Implement `IterWeightedEdges` for `[HashMap<usize, W>; V]`.
- Implement `Outdegree` for `Vec<HashMap<usize, W>>`.
- Implement `Outdegree` for `[HashMap<usize, W>; V]`.

Removed

- Remove `VertexWeight` trait.

## [0.4.2] - 2024-04-03

Changed

- Rename `*::shortest_paths` to `predecessors`.

## [0.4.1] - 2024-04-03

Added

- Add `algo::bfs::shortest_paths`.

## [0.4.0] - 2024-04-03

Added

- Add `algo::dijkstra::shortest_paths`.

Changed

- Move `algo::dijkstra::weighted` to `algo::dijkstra`.
- Remove `algo::dijkstra::unweighted`.

## [0.3.3] - 2024-04-02

Added

- Add `algo::bfs::min_distances_single_source`.
- Add `algo::bfs::min_distances`.
- Add benchmarks for `algo::bfs::*`

## [0.3.2] - 2024-04-01

Added

- Add `algo::dijkstra::unweighted::shortest_paths`.
- Test `algo::dijkstra::unweighted::min_distances` with multiple source vertices.
- Test `algo::dijkstra::weighted::min_distances` without sources.

## [0.3.1] - 2024-04-01

Changed

- Update function names in `README`.

## [0.3.0] - 2024-04-01

Added

- Add `algo::dijkstra::unweighted::min_distances_single_source`.
- Add doctest example for `algo::dijkstra::unweighted::min_distances_single_source`
- Add doctest example for `algo::dijkstra::unweighted::min_distances`
- Add doctest example for `algo::dijkstra::weighted::min_distances_single_source`.

Changed

- Move `algo::DijkstraUnweighted::dijkstra` to `algo::dijkstra::unweighted::min_distances`.
- Move `algo::DijkstraWeighted::dijkstra` to `algo::dijkstra::weighted::min_distances`.
- Move `algo::dijkstra::dijkstra_sssp_weighted` to `algo::dijkstra::weighted::min_distances_single_source`.

Removed

- Remove `new` benchmark.

## [0.2.3] - 2024-03-31

Added

- Add GitHub Action on push to main and PRs on main.
- Add `dijkstra_sssp_weighted`.

Changed

- Make `CHANGELOG.md` adhere to [keep a changelog](https://keepachangelog.com/en/1.0.0/).
- Test `Dijkstra.Unweighted.dijkstra_sssp_unweighted` for every source vertex.

## [0.2.2] - 2024-03-31

Added

- Add doctest example for `Weighted.dijkstra`.

Fixed

- Fix trait descriptions in `README.md`.

## [0.2.1] - 2024-03-31

Added

- Add `dijkstra_sssp_unweighted`.
- Add missing documentation for the public API.

Changed

- Enable selected lints from `restriction` group.
- Export `algo`, `op`, and `repr` modules.
- Group lints into groups, restrictions, `rustdoc`, and `rustc` lints.
- Use `core` and `alloc` imports over `std` where possible.

Fixed

- Make descriptions more consistent.

## [0.2.0] - 2024-03-30

Changed

- Rename `Indegree.in_degree` to `Indegree.indegree`.
- Rename `Outdegree.out_degree` to `Outdegree.outdegree`.

## [0.1.0] - 2024-03-30

- Initial release
