# Changelog

## Provisional roadmap

- Add `IterAllArcsMut` trait.
- Add `IterAllWeightedArcsMut` trait.
- Add `bfs::shortest_paths`.
- Add `bfs::single_pair_shortest_paths`.
- Add `dijkstra::shortest_paths`.
- Add `dijkstra::single_pair_shortest_paths`.
- Add a `dfs` module.
- Add a generator for single-vertex graphs.
- Add a generator for single-sink complete graphs.
- Add a generator for single-source complete graphs.
- Benchmark against popular Rust graph libraries.
- Benchmark against popular graph libraries in other languages.
- Release via GitHub Actions.
- Speed up GHA workflow.

## [0.47.2] - 2024-05-19

Added

- Add documentation alias "in_degree" for `op::indegree`.
- Add documentation alias "out_degree" for `op::outdegree`.

Removed

- Remove testing from `publish` script.

## [0.47.1] - 2024-05-18

Added

- Add alias `singleton` for `trivial`.
- Add method `trivial`.
- Add unit test `btree_map_btree_map_trivial` for `trivial`.
- Add unit test `btree_map_btree_set_unweighted_trivial` for `trivial`.
- Add unit test `btree_map_btree_set_weighted_trivial` for `trivial`.
- Add unit test `btree_map_vec_unweighted_trivial` for `trivial`.
- Add unit test `btree_map_vec_weighted_trivial` for `trivial`.
- Add unit test `btree_set_tuple_unweighted_trivial` for `trivial`.
- Add unit test `btree_set_tuple_weighted_trivial` for `trivial`.
- Add unit test `hash_map_hash_map_trivial` for `trivial`.
- Add unit test `hash_map_hash_set_unweighted_trivial` for `trivial`.
- Add unit test `hash_map_hash_set_weighted_trivial` for `trivial`.
- Add unit test `hash_map_vec_unweighted_trivial` for `trivial`.
- Add unit test `hash_map_vec_weighted_trivial` for `trivial`.
- Add unit test `hash_set_tuple_unweighted_trivial` for `trivial`.
- Add unit test `hash_set_tuple_weighted_trivial` for `trivial`.
- Add unit test `vec_btree_set_unweighted_trivial` for `trivial`.
- Add unit test `vec_btree_set_weighted_trivial` for `trivial`.
- Add unit test `vec_hash_set_unweighted_trivial` for `trivial`.
- Add unit test `vec_hash_set_weighted_trivial` for `trivial`.
- Add unit test `vec_tuple_unweighted_trivial` for `trivial`.
- Add unit test `vec_tuple_weighted_trivial` for `trivial`.
- Add unit test `vec_vec_unweighted_trivial` for `trivial`.
- Add unit test `vec_vec_unweighted_trivial` for `trivial`.
- Add unit test `vec_vec_weighted_trivial` for `trivial`.

Changed

- Mark `Complete::complete` as `#[must_use]`.
- Mark `CompleteConst::complete` as `#[must_use]`.
- Mark `Cycle::cycle` as `#[must_use]`.
- Mark `CycleConst::cycle` as `#[must_use]`.
- Mark `Empty::empty` as `#[must_use]`.
- Mark `EmptyConst::empty` as `#[must_use]`.
- Mark `RandomTournament::random_tournament` as `#[must_use]`.

## [0.47.0] - 2024-05-18

Added

- Add `RandomTournament` example to `README`.
- Add `gen::RandomTournament` generator.
- Add `prng::SplitMix64` PRNG to seed `Xoshiro256StarStar`.
- Add `prng::Xoshiro256StarStar` PRNG.
- Add `prng` module.
- Add unit test `btree_map_btree_set` for `RandomTournament`.
- Add unit test `btree_map_vec` for `RandomTournament`.
- Add unit test `hash_map_hash_set` for `RandomTournament`.
- Add unit test `hash_map_vec` for `RandomTournament`.
- Add unit test `vec_btree_set` for `RandomTournament`.
- Add unit test `vec_hash_set` for `RandomTournament`.
- Add unit test `vec_vec` for `RandomTournament`.
- Implement `RandomTournament` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `RandomTournament` for `BTreeMap<usize, Vec<usize>>`.
- Implement `RandomTournament` for `HashMap<usize, HashSet<usize>>`.
- Implement `RandomTournament` for `HashMap<usize, Vec<usize>>`.
- Implement `RandomTournament` for `Vec<BTreeSet<usize>>`.
- Implement `RandomTournament` for `Vec<HashSet<usize>>`.
- Implement `RandomTournament` for `Vec<Vec<usize>>`.

## [0.46.0] - 2024-05-17

Added

- Add `IterArcsMut` trait.
- Add `arr_vec` unit test for `IterArcsMut`.
- Add `btree_map_vec` unit test for `IterArcsMut`.
- Add `hash_map_vec` unit test for `IterArcsMut`.
- Add `slice_vec` unit test for `IterArcsMut`.
- Add `vec_vec` unit test for `IterArcsMut`.
- Implement `IterArcsMut` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterArcsMut` for `HashMap<usize, Vec<usize>>`.
- Implement `IterArcsMut` for `Vec<Vec<usize>>`.
- Implement `IterArcsMut` for `[Vec<usize>; V]`.
- Implement `IterArcsMut` for `[Vec<usize>]`.

## [0.45.1] - 2024-05-17

Fixed

- Fix typo

## [0.45.0] - 2024-05-15

Changed

- Breaking: Rename `*edge*` to `*arc*` in places where it refers to directed edges.

## [0.44.0] - 2024-05-15

Changed

- Breaking: Rename `CountAllVertices` to `Order`.
- Breaking: Rename `CountAllArcs` to `Size`.

## [0.43.0] - 2024-05-13

Added

- Add `IsRegular` trait.
- Add a "How to implement" section to `IsSymmetric` documentation.
- Add prop test `empty_btree_map_btree_map` for `IsRegular`.
- Add prop test `empty_btree_map_btree_set_unweighted` for `IsRegular`.
- Add prop test `empty_hash_map_hash_map` for `IsRegular`.
- Add prop test `empty_hash_map_hash_set_unweighted` for `IsRegular`.
- Add prop test `empty_vec_btree_map` for `IsRegular`.
- Add prop test `empty_vec_btree_set_unweighted` for `IsRegular`.
- Add prop test `empty_vec_hash_map` for `IsRegular`.
- Add prop test `empty_vec_hash_set_unweighted` for `IsRegular`.
- Add unit test `arr_btree_map` for `IsRegular`.
- Add unit test `arr_btree_set_unweighted` for `IsRegular`.
- Add unit test `arr_hash_map` for `IsRegular`.
- Add unit test `arr_hash_set_unweighted` for `IsRegular`.
- Add unit test `btree_map_btree_map()` for `IterVertices`.
- Add unit test `btree_map_btree_map` for `IsRegular`.
- Add unit test `btree_map_btree_set_weighted()` for `IterVertices`.
- Add unit test `btree_map_btree_set` for `IsRegular`.
- Add unit test `btree_map_vec_weighted()` for `IterVertices`.
- Add unit test `hash_map_hash_map()` for `IterVertices`.
- Add unit test `hash_map_hash_map` for `IsRegular`.
- Add unit test `hash_map_hash_set_weighted()` for `IterVertices`.
- Add unit test `hash_map_hash_set` for `IsRegular`.
- Add unit test `hash_map_vec_weighted()` for `IterVertices`.
- Add unit test `slice_btree_map` for `IsRegular`.
- Add unit test `slice_btree_set_unweighted` for `IsRegular`.
- Add unit test `slice_hash_map` for `IsRegular`.
- Add unit test `slice_hash_set_unweighted` for `IsRegular`.
- Add unit test `vec_btree_map` for `IsRegular`.
- Add unit test `vec_btree_set_unweighted` for `IsRegular`.
- Add unit test `vec_hash_map` for `IsRegular`.
- Add unit test `vec_hash_set_unweighted` for `IsRegular`.
- Implement `IsRegular` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsRegular` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsRegular` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsRegular` for `HashMap<usize, HashSet<usize>>`.
- Implement `IsRegular` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsRegular` for `Vec<BTreeSet<usize>>`.
- Implement `IsRegular` for `Vec<HashMap<usize, W>>`.
- Implement `IsRegular` for `Vec<HashSet<usize>>`.
- Implement `IsRegular` for `[BTreeMap<usize, W>; V]`.
- Implement `IsRegular` for `[BTreeMap<usize, W>]`.
- Implement `IsRegular` for `[BTreeSet<usize>; V]`.
- Implement `IsRegular` for `[BTreeSet<usize>]`.
- Implement `IsRegular` for `[HashMap<usize, W>; V]`.
- Implement `IsRegular` for `[HashMap<usize, W>]`.
- Implement `IsRegular` for `[HashSet<usize>; V]`.
- Implement `IsRegular` for `[HashSet<usize>]`.
- Implement `IterVertices` for `BTreeMap<usize, BTreeMap<usize, W>>`
- Implement `IterVertices` for `BTreeMap<usize, BTreeSet<(usize, W)>>`
- Implement `IterVertices` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IterVertices` for `BTreeMap<usize, Vec<(usize, W)>>`
- Implement `IterVertices` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterVertices` for `HashMap<usize, HashMap<usize, W>>`
- Implement `IterVertices` for `HashMap<usize, HashSet<(usize, W)>>`
- Implement `IterVertices` for `HashMap<usize, HashSet<usize>>`.
- Implement `IterVertices` for `HashMap<usize, Vec<(usize, W)>>`
- Implement `IterVertices` for `HashMap<usize, Vec<usize>>`.

## [0.42.4] - 2024-05-12

Added

- Add proptest strategy `simple_v_e`.

Changed

- Simplify `README` examples.

## [0.42.3] - 2024-05-12

Changed

- Improve `binop_vertices` tests.

## [0.42.2] - 2024-05-11

Added

- Add `IsPendant` trait.

## [0.42.1] - 2024-05-11

Added

- Add `Degree` trait.

## [0.42.0] - 2024-05-11

Added

- Add unit test `has_arc` for `AdjacencyMatrix<V>`.
- Implement `HasEdge` for `AdjacencyMatrix<V>`.
- Implement `IsBalanced` for `AdjacencyMatrix<V>`.
- Implement `IsIsolated` for `AdjacencyMatrix<V>`.
- Implement `IsSymmetric` for `AdjacencyMatrix<V>`.
- Implement `IterAllArcs` for `AdjacencyMatrix<V>`.

Changed

- Breaking: `AdjacencyMatrix::<0>::new` now panics.
- Simplify `AdjacencyMatrix<V>` op implementations with `IterVertices`.
- Simplify `AdjacencyMatrix<V>` tests with `Iterator::eq`.
- Simplify `IsSymmetric` implementations with `HasArc`.
- Simplify arguments to `Iterator::eq` in tests.

## [0.41.0] - 2024-05-11

Changed

- Breaking: `Complete::complete` now panics if `v` is zero.
- Breaking: `CompleteConst::complete_const` now panics if `V` is zero.
- Breaking: `Cycle::cycle` now panics if `v` is zero.
- Breaking: `CycleConst::cycle_const` now panics if `V` is zero.
- Breaking: `Empty::empty` now panics if `v` is zero.
- Breaking: `EmptyConst::empty_const` now panics if `V` is zero.

## [0.40.0] - 2024-05-10

Added

- Add documentation alias "in_valence" for `indegree`.
- Add documentation alias "inward_demidegree" for `indegree`.
- Add documentation alias "out_valence" for `outdegree`.
- Add documentation alias "outward_demidegree" for `outdegree`.
- Add unit test `arr_btree_map` for `IsIsolated`.
- Add unit test `arr_btree_set` for `IsIsolated`.
- Add unit test `arr_hash_map` for `IsIsolated`.
- Add unit test `arr_hash_set` for `IsIsolated`.
- Add unit test `btree_map_btree_map` for `IsIsolated`.
- Add unit test `btree_map_btree_set` for `IsIsolated`.
- Add unit test `hash_map_hash_map` for `IsIsolated`.
- Add unit test `hash_map_hash_set` for `IsIsolated`.
- Add unit test `slice_btree_map` for `IsBalanced`.
- Add unit test `slice_btree_map` for `IsIsolated`.
- Add unit test `slice_btree_set` for `IsBalanced`.
- Add unit test `slice_btree_set` for `IsIsolated`.
- Add unit test `slice_hash_map` for `IsBalanced`.
- Add unit test `slice_hash_map` for `IsIsolated`.
- Add unit test `slice_hash_set` for `IsBalanced`.
- Add unit test `slice_hash_set` for `IsIsolated`.
- Add unit test `vec_btree_map` for `IsIsolated`.
- Add unit test `vec_btree_set` for `IsIsolated`.
- Add unit test `vec_hash_map` for `IsIsolated`.
- Add unit test `vec_hash_set` for `IsIsolated`.
- Implement `IsBalanced` for `[BTreeMap<usize, W>]`.
- Implement `IsBalanced` for `[BTreeSet<usize>]`.
- Implement `IsBalanced` for `[HashMap<usize, W>]`.
- Implement `IsBalanced` for `[HashSet<usize>]`.
- Implement `IsIsolated` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsIsolated` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsIsolated` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsIsolated` for `HashMap<usize, HashSet<usize>>`.
- Implement `IsIsolated` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsIsolated` for `Vec<BTreeSet<usize>>`.
- Implement `IsIsolated` for `Vec<HashMap<usize, W>>`.
- Implement `IsIsolated` for `Vec<HashSet<usize>>`.
- Implement `IsIsolated` for `[BTreeMap<usize, W>; V]`.
- Implement `IsIsolated` for `[BTreeMap<usize, W>]`.
- Implement `IsIsolated` for `[BTreeSet<usize>; V]`.
- Implement `IsIsolated` for `[BTreeSet<usize>]`.
- Implement `IsIsolated` for `[HashMap<usize, W>; V]`.
- Implement `IsIsolated` for `[HashMap<usize, W>]`.
- Implement `IsIsolated` for `[HashSet<usize>; V]`.
- Implement `IsIsolated` for `[HashSet<usize>]`.

Changed

- Restrict existing documentation aliases to module level.
- Simplify `AddArc` "Panics" sections.
- Simplify `Indegree` tests.
- Simplify `IsBalanced` tests.
- Simplify `Outdegree` tests.

Fixed

- Breaking: Fix `IsBalanced`.
- Fix `complete` documentation aliases.
- Fix `empty_const` documentation aliases.

## [0.39.0] - 2024-05-09

Changed

- Breaking: `AddArc for BTreeMap<usize, BTreeSet<usize>>` no longer panics.
- Breaking: `AddArc for BTreeMap<usize, Vec<usize>>` no longer panics.
- Breaking: `AddArc for HashMap<usize, HashSet<usize>>` no longer panics.
- Breaking: `AddArc for HashMap<usize, Vec<usize>>` no longer panics.
- Breaking: `AddWeightedArc for BTreeMap<usize, BTreeMap<usize, W>>` no longer panics.
- Breaking: `AddWeightedArc for BTreeMap<usize, BTreeSet<(usize, W)>>` no longer panics.
- Breaking: `AddWeightedArc for BTreeMap<usize, Vec<(usize, W)>>` no longer panics.
- Breaking: `AddWeightedArc for HashMap<usize, HashMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for BTreeMap<usize, BTreeMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for BTreeMap<usize, BTreeSet<usize>>` no longer panics.
- Breaking: `RemoveArc for HashMap<usize, HashMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for HashMap<usize, HashSet<usize>>` no longer panics.

Removed

- Remove panic documentation from `Size` implementations.

## [0.38.2] - 2024-05-09

Added

- Add property test `binop_vertices_bounds` for `binop_vertices`.
- Add unit test `arr_vec_parallel_arcs` for `IsSimple`.
- Add unit test `arr_vec_self_loop` for `IsSimple`.
- Add unit test `arr_vec_simple` for `IsSimple`.
- Add unit test `slice_vec_parallel_arcs` for `IsSimple`.
- Add unit test `slice_vec_self_loop` for `IsSimple`.
- Add unit test `slice_vec_simple` for `IsSimple`.
- Add unit test `vec_vec_parallel_arcs` for `IsSimple`.
- Add unit test `vec_vec_self_loop` for `IsSimple`.
- Add unit test `vec_vec_simple` for `IsSimple`.
- Implement `IsSimple` for `Vec<Vec<usize>>`.
- Implement `IsSimple` for `[Vec<usize>; V]`.
- Implement `IsSimple` for `[Vec<usize>]`.

## [0.38.1] - 2024-05-09

Changed

- Fix formatting

## [0.38.0] - 2024-05-09

Added

- Add documentation alias "isograph" for `IsBalanced`.
- Add documentation alias "pseudosymmetric" for `IsBalanced`.
- Add trait `HasEdge`.
- Add trait `IsSymmetric`.
- Implement `HasEdge` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `HasEdge` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `HasEdge` for `BTreeSet<(usize, usize)>`.
- Implement `HasEdge` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `HasEdge` for `HashMap<usize, HashSet<usize>>`.
- Implement `HasEdge` for `HashSet<(usize, usize)>`.
- Implement `HasEdge` for `Vec<BTreeMap<usize, W>>`.
- Implement `HasEdge` for `Vec<BTreeSet<usize>>`.
- Implement `HasEdge` for `Vec<HashMap<usize, W>>`.
- Implement `HasEdge` for `Vec<HashSet<usize>>`.
- Implement `HasEdge` for `[BTreeMap<usize, W>; V]`.
- Implement `HasEdge` for `[BTreeMap<usize, W>]`.
- Implement `HasEdge` for `[BTreeSet<usize>; V]`.
- Implement `HasEdge` for `[BTreeSet<usize>]`.
- Implement `HasEdge` for `[HashMap<usize, W>; V]`.
- Implement `HasEdge` for `[HashMap<usize, W>]`.
- Implement `HasEdge` for `[HashSet<usize>; V]`.
- Implement `HasEdge` for `[HashSet<usize>]`.
- Implement `IsSymmetric` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsSymmetric` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsSymmetric` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsSymmetric` for `HashMap<usize, HashSet<usize>>`.
- Implement `IsSymmetric` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsSymmetric` for `Vec<BTreeSet<usize>>`.
- Implement `IsSymmetric` for `Vec<HashMap<usize, W>>`.
- Implement `IsSymmetric` for `Vec<HashSet<usize>>`.
- Implement `IsSymmetric` for `[BTreeMap<usize, W>; V]`.
- Implement `IsSymmetric` for `[BTreeMap<usize, W>]`.
- Implement `IsSymmetric` for `[BTreeSet<usize>; V]`.
- Implement `IsSymmetric` for `[BTreeSet<usize>]`.
- Implement `IsSymmetric` for `[HashMap<usize, W>; V]`.
- Implement `IsSymmetric` for `[HashMap<usize, W>]`.
- Implement `IsSymmetric` for `[HashSet<usize>; V]`.
- Implement `IsSymmetric` for `[HashSet<usize>]`.
- Implement `IterAllWeightedArcs` for `[BTreeMap<usize, W>]`.
- Implement `IterAllWeightedArcs` for `[BTreeSet<(usize, W)>]`.
- Implement `IterAllWeightedArcs` for `[HashMap<usize, W>]`.
- Implement `IterAllWeightedArcs` for `[HashSet<(usize, W)>]`.
- Implement `IterAllWeightedArcs` for `[Vec<(usize, W)>]`.

Changed

- Breaking: Rename `add_weighted_arc_is_arc` to `add_weighted_arc_has_arc`.
- Breaking: Rename `add_arc_remove_arc` to `add_arc_has_arc`.
- Breaking: Rename `IsArc` to `HasArc`.

Removed

- Remove bound `W: Ord` from `HasArc for [BTreeMap<usize, W>]`.
- Remove bound `W: Ord` from `HasArc for [BTreeMap<usize, W>; V]`.

## [0.37.0] - 2024-05-08

Added

- Implement `IsBalanced` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsBalanced` for `Vec<HashMap<usize, W>>`.
- Implement `IsBalanced` for `[BTreeMap<usize, W>; V]`.
- Implement `IsBalanced` for `[HashMap<usize, W>; V]`.
- Implement `IterAllWeightedArcs` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IterAllWeightedArcs` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `IterAllWeightedArcs` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `IterAllWeightedArcs` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IterAllWeightedArcs` for `HashMap<usize, HashSet<(usize, W)>>`.
- Implement `IterAllWeightedArcs` for `HashMap<usize, Vec<(usize, W)>>`.
- Implement `IterAllWeightedArcs` for `Vec<BTreeMap<usize, W>>`.
- Implement `IterAllWeightedArcs` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `IterAllWeightedArcs` for `Vec<HashMap<usize, W>>`.
- Implement `IterAllWeightedArcs` for `Vec<HashSet<(usize, W)>>`.
- Implement `IterAllWeightedArcs` for `Vec<Vec<(usize, W)>>`.
- Implement `IterAllWeightedArcs` for `[BTreeMap<usize, W>; V]`.
- Implement `IterAllWeightedArcs` for `[BTreeSet<(usize, W)>; V]`.
- Implement `IterAllWeightedArcs` for `[HashMap<usize, W>; V]`.
- Implement `IterAllWeightedArcs` for `[HashSet<(usize, W)>; V]`.
- Implement `IterAllWeightedArcs` for `[Vec<(usize, W)>; V]`.

## [0.36.0] - 2024-05-08

Added

- Add `IsBalanced` trait.
- Implement `IsBalanced` for `Vec<BTreeSet<usize>>`.
- Implement `IsBalanced` for `Vec<HashSet<usize>>`.
- Implement `IsBalanced` for `[BTreeSet<usize>; V]`.
- Implement `IsBalanced` for `[HashSet<usize>; V]`.
- Implement `IterAllArcs` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IterAllArcs` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterAllArcs` for `HashMap<usize, HashSet<usize>>`.
- Implement `IterAllArcs` for `HashMap<usize, Vec<usize>>`.
- Implement `IterAllArcs` for `Vec<BTreeSet<usize>>`.
- Implement `IterAllArcs` for `Vec<HashSet<usize>>`.
- Implement `IterAllArcs` for `Vec<Vec<usize>>`.
- Implement `IterAllArcs` for `[BTreeSet<usize>; V]`.
- Implement `IterAllArcs` for `[BTreeSet<usize>]`.
- Implement `IterAllArcs` for `[HashSet<usize>; V]`.
- Implement `IterAllArcs` for `[HashSet<usize>]`.
- Implement `IterAllArcs` for `[Vec<usize>; V]`.
- Implement `IterAllArcs` for `[Vec<usize>]`.

## [0.35.6] - 2024-05-07

Added

- Add back the description of operations in `README`.

## [0.35.5] - 2024-05-07

Changed

- Clean up `Complete` property tests.
- Clean up `Cycle` property tests.
- Clean up `Empty` property tests.
- Simplify `CompleteConst::complete` implementations with `Empty`.
- Simplify `Cycle::cycle` implementations with `Empty`.
- Simplify `CycleConst::cycle` implementations with `Empty`.

Fixed

- Fix `CompleteConst` `outdegree` unit tests.

## [0.35.4] - 2024-05-07

Added

- Add `btree_set` unit test for `iter_all_weighted_arcs`.

Changed

- Simplify tests with `Iterator::eq`.

## [0.35.3] - 2024-05-06

Changed

- Update `README` examples.

## [0.35.2] - 2024-05-05

Added

- Add `gen::EmptyConst` generator.

## [0.35.1] - 2024-05-05

Added

- Add `gen::Empty` generator.
- Add `publish` script.
- Add documentation alias "circular" for `gen::cyle_const`.
- Add documentation alias "circular" for `gen::cyle`.

Changed

- Simplify implementations of `Complete::complete`.
- Simplify implementations of `CompleteConst::complete_const`.

Removed

- Remove `pre-commit.sh`.
- Remove `setup-pre-commit.sh`.

## [0.35.0] - 2024-05-05

Added

- Add `CompleteConst` unit test `arr_btree_set`.
- Add `CompleteConst` unit test `arr_hash_set`.
- Add `CompleteConst` unit test `arr_vec`.
- Add `CompleteConst` unit test `size_arr_btree_set`.
- Add `CompleteConst` unit test `size_arr_hash_set`.
- Add `CompleteConst` unit test `size_arr_vec`.
- Add `CompleteConst` unit test `order_arr_btree_set`.
- Add `CompleteConst` unit test `order_arr_hash_set`.
- Add `CompleteConst` unit test `order_arr_vec`.
- Add `CompleteConst` unit test `indegree_arr_btree_set`.
- Add `CompleteConst` unit test `indegree_arr_hash_set`.
- Add `CompleteConst` unit test `is_simple_arr_btree_set`.
- Add `CompleteConst` unit test `is_simple_arr_hash_set`.
- Add `CompleteConst` unit test `outdegree_arr_btree_set`.
- Add `CompleteConst` unit test `outdegree_arr_hash_set`.
- Add `CompleteConst` unit test `outdegree_arr_vec`.
- Add `Complete` unit test `size_btree_map_btree_set_0`.
- Add `Complete` unit test `size_btree_map_vec_0`.
- Add `Complete` unit test `size_hash_map_hash_set_0`.
- Add `Complete` unit test `size_hash_map_vec_0`.
- Add `Complete` unit test `size_vec_btree_set_0`.
- Add `Complete` unit test `size_vec_hash_set_0`.
- Add `Complete` unit test `size_vec_vec_0`.
- Add `Complete` unit test `indegree_btree_map_btree_set_0`.
- Add `Complete` unit test `indegree_hash_map_hash_set_0`.
- Add `Complete` unit test `indegree_vec_btree_set_0`.
- Add `Complete` unit test `indegree_vec_hash_set_0`.
- Add `Complete` unit test `outdegree_btree_map_btree_set_0`.
- Add `Complete` unit test `outdegree_btree_map_vec_0`.
- Add `Complete` unit test `outdegree_hash_map_hash_set_0`.
- Add `Complete` unit test `outdegree_hash_map_vec_0`.
- Add `Complete` unit test `outdegree_vec_btree_set_0`.
- Add `Complete` unit test `outdegree_vec_hash_set_0`.
- Add `Complete` unit test `outdegree_vec_vec_0`.

Changed

- Breaking: `Outdegree::outdegree` no longer panics if `s` is not in the graph.

## [0.34.1] - 2024-05-05

Added

- Add the `gen::Complete` generator.
- Add the `gen::CompleteConst` generator.

## [0.34.0] - 2024-05-05

Added

- Add documentation alias "circular" for `gen::Cycle`.
- Add documentation alias "circular" for `gen::CycleConst`.

Changed

- Emphasize the focus on directed graphs in the documentation.

Removed

- Remove `gen::Star`.
- Remove `gen::StarConst`.
- Remove `gen::Linear`
- Remove `gen::LinearConst`.
- Remove `op::Target`.

## [0.33.1] - 2024-05-04

Added

- Add `gen::StarConst` generator.
- Add `gen::StarConst` unit test `arr_btree_set`.
- Add `gen::StarConst` unit test `arr_hash_set`.
- Add `gen::StarConst` unit test `arr_vec`.
- Add `gen::StarConst` unit test `is_simple_arr_btree_set`.
- Add `gen::StarConst` unit test `is_simple_arr_hash_set`.
- Implement `StarConst` for `[BTreeSet<usize>; V]`.
- Implement `StarConst` for `[HashSet<usize>; V]`.
- Implement `StarConst` for `[Vec<usize>; V]`.

## [0.33.0] - 2024-05-04

Added

- Add `gen::Star` property test `indegree_btree_map_btree_set_1`.
- Add `gen::Star` property test `indegree_hash_map_hash_set_1`.
- Add `gen::Star` property test `indegree_vec_btree_set_1`.
- Add `gen::Star` property test `indegree_vec_hash_set_1`.
- Add `gen::Star` property test `outdegree_btree_map_btree_set_1`.
- Add `gen::Star` property test `outdegree_btree_map_vec_1`.
- Add `gen::Star` property test `outdegree_hash_map_hash_set_1`.
- Add `gen::Star` property test `outdegree_hash_map_vec_1`.
- Add `gen::Star` property test `outdegree_vec_btree_set_1`.
- Add `gen::Star` property test `outdegree_vec_hash_set_1`.
- Add `gen::Star` property test `outdegree_vec_vec_1`.
- Link to `CHANGELOG.md` from `README.md`.

Changed

- Breaking: Return an empty arc list for the one vertex in `gen::Star::star(1)`.

## [0.32.3] - 2024-05-04

Added

- Add `gen::Star` property test `size_btree_map_btree_set`.
- Add `gen::Star` property test `size_btree_map_vec`.
- Add `gen::Star` property test `size_hash_map_hash_set`.
- Add `gen::Star` property test `size_hash_map_vec`.
- Add `gen::Star` property test `size_vec_btree_set`.
- Add `gen::Star` property test `size_vec_hash_set`.
- Add `gen::Star` property test `size_vec_vec`.
- Add `gen::Star` property test `order_vec_btree_set`.
- Add `gen::Star` property test `order_vec_hash_set`.
- Add `gen::Star` property test `order_vec_vec`.
- Add `gen::Star` property test `indegree_btree_map_btree_set`.
- Add `gen::Star` property test `indegree_hash_map_hash_set`.
- Add `gen::Star` property test `indegree_vec_btree_set`.
- Add `gen::Star` property test `indegree_vec_hash_set`.
- Add `gen::Star` property test `is_simple_vec_btree_set`.
- Add `gen::Star` property test `is_simple_vec_hash_set`.
- Add `gen::Star` property test `is_simple_vec_vec`.
- Add `gen::Star` property test `outdegree_btree_map_btree_set`.
- Add `gen::Star` property test `outdegree_btree_map_vec`.
- Add `gen::Star` property test `outdegree_hash_map_hash_set`.
- Add `gen::Star` property test `outdegree_hash_map_vec`.
- Add `gen::Star` property test `outdegree_vec_btree_set`.
- Add `gen::Star` property test `outdegree_vec_hash_set`.
- Add `gen::Star` property test `outdegree_vec_vec`.
- Add `gen::Star` unit test `btree_map_btree_set`.
- Add `gen::Star` unit test `btree_map_vec`.
- Add `gen::Star` unit test `hash_map_hash_set`.
- Add `gen::Star` unit test `hash_map_vec`.
- Add `gen::Star` unit test `vec_btree_set`.
- Add `gen::Star` unit test `vec_hash_set`.
- Add `gen::Star` unit test `vec_vec`.
- Add documentation alias for `bfs::single_pair_shortest_path`.
- Add documentation alias for `dijkstra::single_pair_shortest_path`.
- Add generator `gen::Star` for star graphs.
- Implement `gen::Star` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `gen::Star` for `BTreeMap<usize, Vec<usize>>`.
- Implement `gen::Star` for `HashMap<usize, HashSet<usize>>`.
- Implement `gen::Star` for `HashMap<usize, Vec<usize>>`.
- Implement `gen::Star` for `Vec<BTreeSet<usize>>`.
- Implement `gen::Star` for `Vec<HashSet<usize>>`.
- Implement `gen::Star` for `Vec<Vec<usize>>`.

## [0.32.2] - 2024-05-02

Changed

- Add diagram to `README` `bfs::single_pair_shortest_path` example.
- Standardize links in documentation to conform to [RFC-1574](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html#appendix-a-full-conventions-text).

## [0.32.1] - 2024-05-02

Changed

- Standardize documentation summary sentences to conform to [RFC-1574](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html#appendix-a-full-conventions-text).

## [0.32.0] - 2024-05-02

Changed

- Add MIT license.

## [0.31.0] - 2024-05-01

Added

- Add `dijkstra::single_pair_shortest_path`.

Fixed

- Fix `dijkstra::shortest_path`.

## [0.30.9] - 2024-05-01

Added

- Add `Size` unit test `btree_set_tuple_unweighted`.
- Add `Size` unit test `btree_set_tuple_weighted`.
- Add `Size` unit test `hash_set_tuple_unweighted`.
- Add `Size` unit test `hash_set_tuple_weighted`.
- Add `Size` unit test `vec_tuple_unweighted`.
- Add `Size` unit test `vec_tuple_weighted`.
- Add `CycleConst` unit test `arr_tuple`.
- Add `CycleConst` unit test `is_simple_arr_btree_set`.
- Add `CycleConst` unit test `is_simple_arr_hash_set`.
- Add `CycleConst` unit test `is_simple_arr_tuple`.
- Add `Cycle` property test `size_btree_set_tuple`.
- Add `Cycle` property test `size_hash_set_tuple`.
- Add `Cycle` property test `size_vec_tuple`.
- Add `Cycle` property test `is_simple_btree_set_tuple`.
- Add `Cycle` property test `is_simple_hash_set_tuple`.
- Add `Cycle` property test `is_simple_vec_btree_set`.
- Add `Cycle` property test `is_simple_vec_hash_set`.
- Add `Cycle` property test `is_simple_vec_tuple`.
- Add `Cycle` unit test `btree_set_tuple`.
- Add `Cycle` unit test `hash_set_tuple`.
- Add `Cycle` unit test `is_simple_btree_set_tuple_0`.
- Add `Cycle` unit test `is_simple_btree_set_tuple_1`.
- Add `Cycle` unit test `is_simple_hash_set_tuple_0`.
- Add `Cycle` unit test `is_simple_hash_set_tuple_1`.
- Add `Cycle` unit test `is_simple_vec_btree_set_0`.
- Add `Cycle` unit test `is_simple_vec_btree_set_1`.
- Add `Cycle` unit test `is_simple_vec_hash_set_0`.
- Add `Cycle` unit test `is_simple_vec_hash_set_1`.
- Add `Cycle` unit test `is_simple_vec_tuple_0`.
- Add `Cycle` unit test `is_simple_vec_tuple_1`.
- Add `Cycle` unit test `vec_tuple`.
- Add `LinearConst` unit test `is_simple_arr_btree_set`.
- Add `LinearConst` unit test `is_simple_arr_hash_set`.
- Add `Linear` property test `is_simple_vec_btree_set`.
- Add `Linear` property test `is_simple_vec_hash_set`.
- Implement `Size` for `BTreeSet<(usize, usize)>`.
- Implement `Size` for `BTreeSet<(usize, usize, W)>`.
- Implement `Size` for `HashSet<(usize, usize)>`.
- Implement `Size` for `HashSet<(usize, usize, W)>`.
- Implement `Size` for `Vec<(usize, usize)>`.
- Implement `Size` for `Vec<(usize, usize, W)>`.
- Implement `CycleConst` for `[(usize, usize); V]`.
- Implement `Cycle` for `BTreeSet<(usize, usize)>`.
- Implement `Cycle` for `HashSet<(usize, usize)>`.
- Implement `Cycle` for `Vec<(usize, usize)>`.

## [0.30.8] - 2024-04-30

Changed

- Link to `CycleConst` from the `Cycle` documentation.

## [0.30.7] - 2024-04-30

Added

- Implement `CycleConst` for `[BTreeSet<usize>; V]`.
- Implement `CycleConst` for `[HashSet<usize>; V]`.
- Implement `CycleConst` for `[Vec<usize>; V]`.

## [0.30.6] - 2024-04-30

Changed

- Set `opt-level` to `3` for `proptest`.

## [0.30.5] - 2024-04-29

Added

- Add `gen::Cycle` property test `size_btree_map_btree_set`.
- Add `gen::Cycle` property test `size_btree_map_vec`.
- Add `gen::Cycle` property test `size_hash_map_hash_set`.
- Add `gen::Cycle` property test `size_hash_map_vec`.
- Add `gen::Cycle` property test `size_vec_btree_set`.
- Add `gen::Cycle` property test `size_vec_hash_set`.
- Add `gen::Cycle` property test `size_vec_vec`.
- Add `gen::Cycle` property test `indegree_btree_map_btree_set`.
- Add `gen::Cycle` property test `indegree_hash_map_hash_set`.
- Add `gen::Cycle` property test `outdegree_btree_map_btree_set`.
- Add `gen::Cycle` property test `outdegree_btree_map_vec`.
- Add `gen::Cycle` property test `outdegree_hash_map_hash_set`.
- Add `gen::Cycle` property test `outdegree_hash_map_vec`.
- Add `gen::Cycle` unit test `btree_map_btree_set`.
- Add `gen::Cycle` unit test `btree_map_vec`.
- Add `gen::Cycle` unit test `hash_map_hash_set`.
- Add `gen::Cycle` unit test `hash_map_vec`.
- Add `gen::Linear` property test `size_btree_map_btree_set`.
- Add `gen::Linear` property test `size_btree_map_vec`.
- Add `gen::Linear` property test `size_hash_map_hash_set`.
- Add `gen::Linear` property test `size_hash_map_vec`.
- Add `gen::Linear` property test `size_vec_btree_set`.
- Add `gen::Linear` property test `size_vec_hash_set`.
- Add `gen::Linear` property test `size_vec_vec`.
- Add `gen::Linear` property test `order_vec_btree_set`.
- Add `gen::Linear` property test `order_vec_hash_set`.
- Add `gen::Linear` property test `order_vec_vec`.
- Add `gen::Linear` property test `indegree_btree_map_btree_set`.
- Add `gen::Linear` property test `indegree_hash_map_hash_set`.
- Add `gen::Linear` property test `indegree_vec_btree_set`.
- Add `gen::Linear` property test `indegree_vec_hash_set`.
- Add `gen::Linear` property test `outdegree_btree_map_btree_set`.
- Add `gen::Linear` property test `outdegree_btree_map_vec`.
- Add `gen::Linear` property test `outdegree_hash_map_hash_set`.
- Add `gen::Linear` property test `outdegree_hash_map_vec`.
- Add `gen::Linear` property test `outdegree_vec_btree_set`.
- Add `gen::Linear` property test `outdegree_vec_hash_set`.
- Add `gen::Linear` property test `outdegree_vec_vec`.
- Implement `Cycle` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Cycle` for `BTreeMap<usize, Vec<usize>>`.
- Implement `Cycle` for `HashMap<usize, HashSet<usize>>`.
- Implement `Cycle` for `HashMap<usize, Vec<usize>>`.
- Implement `ArcWeight` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `ArcWeight` for `Vec<BTreeMap<usize, W>>`.
- Implement `ArcWeight` for `[BTreeMap<usize, W>; V]`.
- Implement `ArcWeight` for `[BTreeMap<usize, W>]`.
- Implement `Linear` for `HashMap<usize, HashSet<usize>>`.
- Implement `Linear` for `HashMap<usize, Vec<usize>>`.

## [0.30.4] - 2024-04-29

Added

- Add `gen::Cycle` property test `order_vec_vec`.
- Add `gen::Cycle` property test `order_vec_btree_set`.
- Add `gen::Cycle` property test `order_vec_hash_set`.
- Add `gen::Cycle` property test `indegree_vec_btree_set`.
- Add `gen::Cycle` property test `indegree_vec_hash_set`.
- Add `gen::Cycle` property test `outdegree_vec_vec`.
- Add `gen::Cycle` property test `outdegree_vec_btree_set`.
- Add `gen::Cycle` property test `outdegree_vec_hash_set`.

## [0.30.3] - 2024-04-29

Changed

- Clean up examples in `README`.

## [0.30.2] - 2024-04-29

Changed

- Add more extensive examples to `README`.
- Simplify `strategy::binop_vertices` implementation.

## [0.30.1] - 2024-04-28

Fixed

- Fix `prop::strategy::binop_vertices` when `v` is zero inside `flat_map`.

## [0.30.0] - 2024-04-28

Removed

- Breaking: Remove `prop::remove_arc_is_arc`.

## [0.29.1] - 2024-04-28

Added

- Add `add_weighted_arc_is_arc` property test `btree_map_btree_set`.
- Add `add_weighted_arc_is_arc` property test `hash_map_hash_set`.
- Add `add_weighted_arc_is_arc` property test `slice_btree_set`.
- Add `add_weighted_arc_is_arc` property test `slice_hash_set`.
- Add `add_weighted_arc_is_arc` property test `vec_btree_map`.
- Add `add_weighted_arc_is_arc` property test `vec_hash_map`.
- Add `add_weighted_arc_is_arc` unit test `arr_btree_map`.
- Add `add_weighted_arc_is_arc` unit test `arr_hash_map`.
- Add `add_weighted_arc_remove_arc` property test `btree_map_btree_map`.
- Add `add_weighted_arc_remove_arc` property test `hash_map_hash_map`.
- Add `add_weighted_arc_remove_arc` property test `slice_btree_map`.
- Add `add_weighted_arc_remove_arc` property test `slice_hash_map`.
- Add `add_weighted_arc_remove_arc` property test `vec_btree_map`.
- Add `add_weighted_arc_remove_arc` property test `vec_hash_map`.
- Add `add_weighted_arc_remove_arc` unit test `arr_btree_map`.
- Add `add_weighted_arc_remove_arc` unit test `arr_hash_map`.

Changed

- Loosen bound on `G` in `add_weighted_arc_is_arc` to `?Sized`.

Fixed

- Fix `prop::strategy::binop_vertices` when `v` is zero.

Removed

- Remove `add_weighted_arc_is_arc` unit test `vec_hash_map`.
- Remove `add_weighted_arc_remove_arc` unit test `vec_hash_map`.
- Rename `prop::strategy::v_s_t` to `prop::strategy::binop_vertices`.

## [0.29.0] - 2024-04-28

Added

- Add `add_arc_is_arc` property test `btree_map_btree_set`.
- Add `add_arc_is_arc` property test `hash_map_hash_set`.
- Add `add_arc_is_arc` property test `slice_btree_set`.
- Add `add_arc_is_arc` property test `slice_hash_set`.
- Add `add_arc_is_arc` property test `vec_btree_set`.
- Add `add_arc_is_arc` property test `vec_hash_set`.
- Add `add_arc_is_arc` unit test `arr_btree_set`.
- Add `add_arc_remove_arc` property test `vec_btree_set`.
- Add `add_arc_remove_arc` property test `vec_hash_set`.
- Add `add_arc_remove_arc` unit test `arr_btree_set`.
- Add `prop::strategy::v_s_t` to order, source, and target.
- Add `proptest` as a dev-dependency.

Changed

- Breaking: Move `op::prop` to `prop`.

Removed

- Remove `add_arc_is_arc` unit test `hash_map_hash_set`.
- Remove `add_arc_is_arc` unit test `hash_map_hash_set`.
- Remove `add_arc_is_arc` unit test `vec_hash_set`.
- Remove `add_arc_remove_arc` unit test `vec_hash_set`.

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
- Bench `add_arc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Bench `add_arc` for `BTreeMap<usize, Vec<usize>>`.
- Bench `add_arc` for `Vec<BTreeSet<usize>>`.
- Bench `add_arc` for `[BTreeSet<usize>; V]`.
- Bench `size` for `Vec<BTreeMap<usize, usize>>`.

Changed

- Breaking: Return immediately in `bfs::shortest_path` when it finds the target, before pushing the target to the queue.
- Compress textual diagrams.
- Cross-link `bfs` and `dijkstra` in module documentation.
- Document "Panics" in `bfs` and `dijkstra`.
- Link to `op` in the documentation of `repr`.
- Remove `min_time` from benches.

## [0.27.1] - 2024-04-26

Changed

- Check the top changelog entry version number on pre-commit.

## [0.27.0] - 2024-04-26

Added

- Add `op::Target`, a trait to get the target vertex of an adjacency list arc.

Changed

- Replace `Linear for G: AddArc + Empty` with `Linear for Vec<BTreeSet<usize>>`.
- Replace `Linear for G: AddArc + Empty` with `Linear for Vec<HashSet<usize>>`.
- Replace `Linear for G: AddArc + Empty` with `Linear for Vec<Vec<usize>>`.

Removed

- Breaking: Remove `gen::Empty`.

## [0.26.1] - 2024-04-26

Added

- Add `gen::Empty`, a generator for empty graphs.
- Implement `Linear` for `G: AddArc + Empty`.
- Implement `Empty` for `Vec<T: Clone + Default + IntoIterator<Item = usize>>`.

Changed

- Replace `Linear for Vec<BTreeSet<usize>>` with `Linear for G: AddArc + Empty`.
- Replace `Linear for Vec<HashSet<usize>>` with `Linear for G: AddArc + Empty`.
- Replace `Linear for Vec<Vec<usize>>` with `Linear for G: AddArc + Empty`.

## [0.26.0] - 2024-04-25

Changed

- Breaking: Remove the `nightly` feature, as only `adjacency_matrix` depends on it.
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

- Add a doctest for `gen::Linear`.

## [0.24.0] - 2024-04-23

Added

- Add `gen::Linear`, a generator for linear graphs.

Changed

- `IsArc` for `[BTreeMap<usize, W>]` no longer panics if `s` is not in the graph.
- `IsArc` for `[BTreeSet<usize>]` no longer panics if `s` is not in the graph.
- `IsArc` for `[HashMap<usize, W>]` no longer panics if `s` is not in the graph.
- `IsArc` for `[HashSet<usize>]` no longer panics if `s` is not in the graph.

## [0.23.1] - 2024-04-23

Added

- Add more tests for `bfs::single_pair_shortest_path`.
- Mention `bfs::single_pair_shortest_path` in the module documentation of `bfs`.

## [0.23.0] - 2024-04-22

Added

- Add `bfs::single_pair_shortest_path`.

Changed

- Breaking: Remove `?Sized` bound from `graph` in `bfs::distances`.
- Breaking: Remove `?Sized` bound from `graph` in `bfs::predecessors`.
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
- Use `HashSet` instead of `AdjacencyMatrix` in the `lib` test.
- Add `Cargo.toml` to `.gitignore`.

## [0.22.0] - 2024-04-21

Changed

- Breaking: Disable `adjacency_matrix` by default to support stable Rust.

## [0.21.0] - 2024-04-21

Added

- Add a `nightly` feature.

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
- Breaking: `predecessor::search` returns a singleton path if the target is the source.

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

- Move the list of standard library graph representations from `README` to `op/mod.rs`.

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

- Remove unused parameter `W` from `remove_arc_is_arc`.

## [0.16.1] - 2024-04-17

Added

- Implement `RemoveArc` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `RemoveArc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `RemoveArc` for `Vec<BTreeMap<usize, W>>`.
- Implement `RemoveArc` for `Vec<BTreeSet<usize>>`.
- Implement `RemoveArc` for `[BTreeMap<usize, W>; V]`.
- Implement `RemoveArc` for `[BTreeMap<usize, W>]`.
- Implement `RemoveArc` for `[BTreeSet<usize>; V]`.
- Implement `RemoveArc` for `[BTreeSet<usize>]`.

Changed

- `RemoveArc::remove_arc` now returns a `bool` indicating whether it removed the arc.

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

- Implement `IterWeightedArcs` for `Vec<BTreeMap<usize, W>>`.
- Implement `IterWeightedArcs` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `IterWeightedArcs` for `[BTreeMap<usize, W>; V]`.
- Implement `IterWeightedArcs` for `[BTreeMap<usize, W>]`.
- Implement `IterWeightedArcs` for `[BTreeSet<(usize, W)>; V]`.
- Implement `IterWeightedArcs` for `[BTreeSet<(usize, W)>]`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, BTreeMap<usize, W>>`.

## [0.15.0] - 2024-04-16

Removed

- Breaking: Remove `IterVertices` for `HashMap<_>`.

## [0.14.2] - 2024-04-16

Added

- Implement `IterArcs` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IterArcs` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterArcs` for `Vec<BTreeSet<usize>>`.
- Implement `IterArcs` for `[BTreeSet<usize>; V]`.
- Implement `IterArcs` for `[BTreeSet<usize>]`.

Fixes

- Fix benchmark imports.

## [0.14.1] - 2024-04-16

Added

- Implement `IsSimple` for `Vec<BTreeSet<usize>>`.
- Implement `IsSimple` for `[BTreeSet<usize>]`.
- Implement `IsSimple` for `[BTreeSet<usize>; V]`.
- Implement `IsSimple` for `BTreeSet<(usize, usize)>`.
- Implement `IsSimple` for `BTreeSet<(usize, usize, W)>`.
- Implement `IterAllArcs` for `BTreeSet<(usize, usize)>`.
- Implement `IterAllWeightedArcs` for `BTreeSet<(usize, usize, W)>`.

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

- Implement `Order` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `Order` for `Vec<HashSet<(usize, W)>>`.
- Implement `Order` for `[BTreeSet<(usize, W)>; V]`.
- Implement `Order` for `[BTreeSet<(usize, W)>]`.
- Implement `Order` for `[HashSet<(usize, W); V]`.
- Implement `Order` for `[HashSet<(usize, W)]`.

Fixed

- Fix `cross_country` benchmark.
- Fix `shortest_path_1` benchmark.
- Fix `small_graph_1` benchmark.

## [0.13.1] - 2024-04-15

Added

- Implement `IsArc` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsArc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsArc` for `BTreeSet<(usize, usize)>`.
- Implement `IsArc` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsArc` for `Vec<BTreeSet<usize>>`.
- Implement `IsArc` for `[BTreeMap<usize, W>; V]`.
- Implement `IsArc` for `[BTreeMap<usize, W>]`.
- Implement `IsArc` for `[BTreeSet<usize>; V]`.
- Implement `IsArc` for `[BTreeSet<usize>]`.

## [0.13.0] - 2024-04-14

Added

- Implement `Order` for `Vec<BTreeMap<usize, W>>`.
- Implement `Order` for `Vec<BTreeSet<usize>>`.
- Implement `Order` for `Vec<HashMap<usize, W>>`.
- Implement `Order` for `Vec<HashSet<usize>>`.
- Implement `Order` for `Vec<Vec<(usize, W)>>`.
- Implement `Order` for `Vec<Vec<usize>>`.
- Implement `Order` for `[BTreeMap<usize, W>; V]`.
- Implement `Order` for `[BTreeMap<usize, W>]`.
- Implement `Order` for `[BTreeSet<usize>; V]`.
- Implement `Order` for `[BTreeSet<usize>]`.
- Implement `Order` for `[HashMap<usize, W>; V]`.
- Implement `Order` for `[HashMap<usize, W>]`.
- Implement `Order` for `[HashSet<usize>; V]`.
- Implement `Order` for `[HashSet<usize>]`.
- Implement `Order` for `[Vec<(usize, W)>; V]`.
- Implement `Order` for `[Vec<(usize, W)>]`.
- Implement `Order` for `[Vec<usize>; V]`.
- Implement `Order` for `[Vec<usize>]`.

Removed

- Breaking: Remove `Order` for `HashMap<_>`.
- Breaking: Remove `Order` for `Vec<T>`.
- Breaking: Remove `Order` for `[T; V]`.
- Breaking: Remove `Order` for `[T]`.

## [0.12.1] - 2024-04-14

Added

- Implement `Size` for `BTreeMap<K, BTreeMap<K, W>>`.
- Implement `Size` for `BTreeMap<K, BTreeSet<T>>`.
- Implement `Size` for `BTreeMap<K, Vec<T>>`.
- Implement `Size` for `Vec<BTreeMap<K, W>>`.
- Implement `Size` for `Vec<BTreeSet<T>>`.
- Implement `Size` for `[BTreeMap<K, W>; V]`.
- Implement `Size` for `[BTreeMap<K, W>]`.
- Implement `Size` for `[BTreeSet<T>; V]`.
- Implement `Size` for `[BTreeSet<T>]`.
- Implement `IsArc` for `Vec<HashMap<usize, W>>`.
- Implement `IsArc` for `[HashMap<usize, W>; V]`.

Removed

- Remove `Ord` bound from trait implementations where possible.

## [0.12.0] - 2024-04-14

Added

- Implement `AddWeightedArc` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `AddWeightedArc` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `AddWeightedArc` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `AddWeightedArc` for `Vec<BTreeMap<usize, W>>`.
- Implement `AddWeightedArc` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `AddWeightedArc` for `[BTreeMap<usize, W>; V]`.
- Implement `AddWeightedArc` for `[BTreeMap<usize, W>]`.
- Implement `AddWeightedArc` for `[BTreeSet<(usize, W)>; V]`.
- Implement `AddWeightedArc` for `[BTreeSet<(usize, W)>]`.

Changed

- Breaking: `AddWeightedArc` for `HashMap<_>` now panics if `s` is not in the graph.

## [0.11.1] - 2024-04-14

Added

- Implement `AddArc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `AddArc` for `BTreeMap<usize, Vec<usize>>`.
- Implement `AddArc` for `Vec<BTreeSet<usize>>`.
- Implement `AddArc` for `[BTreeSet<usize>; V]`.
- Implement `AddArc` for `[BTreeSet<usize>]`.

## [0.11.0] - 2024-04-14

Added back `op` implementations for `Vec` and arrays to simplify use cases.

Added

- Implement `AddArc` for `Vec<HashSet<usize>>`.
- Implement `AddArc` for `Vec<Vec<usizee>>`.
- Implement `AddArc` for `[HashSet<usize>; V]`.
- Implement `AddArc` for `[Vec<usize>; V]`.
- Implement `AddWeightedArc` for `Vec<HashMap<usize, W>>`.
- Implement `AddWeightedArc` for `Vec<HashSet<(usize, W)>>`.
- Implement `AddWeightedArc` for `Vec<Vec<(usize, W)>>`.
- Implement `AddWeightedArc` for `[HashMap<usize, W>; V]`.
- Implement `AddWeightedArc` for `[HashSet<(usize, W)>; V]`.
- Implement `AddWeightedArc` for `[Vec<(usize, W)>; V]`.
- Implement `Size` for `Vec<HashMap<K, W>>`.
- Implement `Size` for `Vec<HashSet<T>>`.
- Implement `Size` for `Vec<Vec<T>>`.
- Implement `Size` for `[HashMap<K, W>; V]`.
- Implement `Size` for `[HashSet<T>; V]`.
- Implement `Size` for `[Vec<T>; V]`.
- Implement `ArcWeight` for `Vec<HashMap<usize, W>>`.
- Implement `ArcWeight` for `[HashMap<usize, W>; V]`.
- Implement `Indegree` for `Vec<HashMap<usize, W>>`.
- Implement `Indegree` for `Vec<HashSet<usize>>`.
- Implement `Indegree` for `[HashMap<usize, W>; V]`.
- Implement `Indegree` for `[HashSet<usize>; V]`.
- Implement `IsArc` for `Vec<HashSet<usize>>`.
- Implement `IsArc` for `[HashMap<usize, W>; V]`.
- Implement `IsSimple` for `Vec<(usize, usize)>`.
- Implement `IsSimple` for `Vec<(usize, usize, W)>`.
- Implement `IsSimple` for `Vec<HashSet<usize>>`.
- Implement `IsSimple` for `[(usize, usize); V]`.
- Implement `IsSimple` for `[(usize, usize, W); V]`.
- Implement `IsSimple` for `[HashSet<usize>; V]`.
- Implement `IterAllArcs` for `Vec<(usize, usize)>`.
- Implement `IterAllArcs` for `[(usize, usize); V]`.
- Implement `IterAllWeightedArcs` for `Vec<(usize, usize, W)>`.
- Implement `IterAllWeightedArcs` for `[(usize, usize, W); V]`.
- Implement `IterVertices` for `Vec<T>`.
- Implement `IterVertices` for `[T; V]`.
- Implement `Outdegree` for `Vec<HashMap<K, W>>`.
- Implement `Outdegree` for `Vec<HashSet<T>>`.
- Implement `Outdegree` for `Vec<Vec<T>>`.
- Implement `Outdegree` for `[HashMap<K, W>; V]`.
- Implement `Outdegree` for `[HashSet<T>; V]`.
- Implement `Outdegree` for `[Vec<T>; V]`.
- Implement `RemoveArc` for `Vec<HashMap<usize, W>>`.
- Implement `RemoveArc` for `Vec<HashSet<usize>>`.
- Implement `RemoveArc` for `[HashMap<usize, W>; V]`.
- Implement `RemoveArc` for `[HashSet<usize>; V]`.
- Test `add_arc_remove_arc` for `AdjacencyMatrix`.
- Test `add_arc_remove_arc` for `Vec<HashSet<usize>>`.
- Test `add_arc_remove_arc` for `[HashSet<usize>; V]`.

Fixed

- Remove stray `W` type parameter in `add_arc_remove_arc`.

## [0.10.0] - 2024-04-12

Added

- Implement `IsSimple` for `AdjacencyMatrix`.

Fixed

- `IsSimple` now checks for parallel arcs in `HashSet<(usize, usize, W)>`.

## [0.9.0] - 2024-04-12

Fixed

- `IsSimple` now checks for parallel arcs in `HashSet<(usize, usize)>`.
- `IsSimple` now checks for parallel arcs in `[(usize, usize)]`.
- `IsSimple` now checks for parallel arcs in `[(usize, usize, W)]`.

## [0.8.4] - 2024-04-11

Fixed

- Add missing property `add_weighted_arc_remove_arc`.

## [0.8.3] - 2024-04-11

Added

- Test `add_arc_is_arc` for implementors of `AddArc` and `IsArc`.

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

- Remove `ArcWeight` for `Vec<HashMap<usize, W>>`.
- Remove `ArcWeight` for `[HashMap<usize, W>; V]`.
- Remove `IterVertices` for `HashSet<T>`.

## [0.8.0] - 2024-04-09

Added

- Add installation instructions to `README`.
- Add example usage to `README`.

Removed

- Remove `AddArc` for `Vec<HashSet<T>>`.
- Remove `AddArc` for `Vec<Vec<T>>`.
- Remove `AddArc` for `[HashSet<T>; V]`.
- Remove `AddArc` for `[Vec<T>; V]`.
- Remove `AddWeightedArc` for `Vec<HashMap<usize, W>>`.
- Remove `AddWeightedArc` for `Vec<HashSet<(usize, W)>>`.
- Remove `AddWeightedArc` for `Vec<Vec<(usize, W)>>`.
- Remove `AddWeightedArc` for `[HashMap<usize, W>; V]`.
- Remove `AddWeightedArc` for `[HashSet<(usize, W)>; V]`.
- Remove `AddWeightedArc` for `[Vec<(usize, W)>; V]`.

Changed

- Change `iter_all_arcs` return type to `impl Iterator<Item = (usize, usize)>`.
- Change `iter_all_weighted_arcs` return type to `impl Iterator<Item = (usize, usize, &W)>`.
- Change `iter_weighted_arcs` return type to `impl Iterator<Item = (usize, &W)>`.

## [0.7.0] - 2024-04-07

Added

- Implement `Size` for `[HashMap<K, W>]`.
- Implement `Size` for `[HashSet<T>]`.
- Implement `Size` for `[Vec<T>]`.
- Implement `Order` for `[T]`.
- Implement `ArcWeight` for `[HashMap<usize, W>]`.
- Implement `Indegree` for `[HashMap<usize, W>]`.
- Implement `Indegree` for `[HashSet<usize>]`.
- Implement `IsArc` for `[HashMap<usize, W>]`.
- Implement `IsArc` for `[HashSet<usize>]`.
- Implement `IterAllArcs` for `[(usize, usize)]`.
- Implement `IterAllWeightedArcs` for `[(usize, usize, W)]`.
- Implement `IterArcs` for `[HashSet<usize>]`.
- Implement `IterArcs` for `[Vec<usize>]`.
- Implement `IterVertices` for `&[T]`.
- Implement `IterWeightedArcs` for `[HashMap<usize, W>]`.
- Implement `IterWeightedArcs` for `[HashSet<(usize, W)>]`.
- Implement `IterWeightedArcs` for `[Vec<(usize, W)>]`.
- Implement `Outdegree` for `[HashMap<K, W>]`.
- Implement `Outdegree` for `[HashSet<T>]`.
- Implement `Outdegree` for `[Vec<T>]`.
- Implement `RemoveArc` for `[HashMap<usize, W>]`.
- Implement `RemoveArc` for `[HashSet<usize>]`.

Changed

- Return `(&'a usize, &'a W)` from `iter_weighted_arcs`.

Removed

- Remove `Size` for `Vec<HashMap<K, W>>`.
- Remove `Size` for `Vec<HashSet<T>>`.
- Remove `Size` for `Vec<Vec<T>>`.
- Remove `Size` for `[HashMap<K, W>; V]`.
- Remove `Size` for `[HashSet<T>; V]`.
- Remove `Size` for `[Vec<T>; V]`.
- Remove `Indegree` for `Vec<HashMap<usize, W>>`.
- Remove `Indegree` for `Vec<HashSet<usize>>`.
- Remove `Indegree` for `[HashMap<usize, W>; V]`.
- Remove `Indegree` for `[HashSet<usize>; V]`.
- Remove `IsArc` for `Vec<HashMap<usize, W>>`.
- Remove `IsArc` for `Vec<HashSet<usize>>`.
- Remove `IsArc` for `[HashMap<usize, W>; V]`.
- Remove `IsArc` for `[HashSet<usize>; V]`.
- Remove `IterAllArcs` for `Vec<(usize, usize)>`.
- Remove `IterAllArcs` for `[(usize, usize); V]`.
- Remove `IterAllWeightedArcs` for `Vec<(usize, usize, W)>`.
- Remove `IterAllWeightedArcs` for `[(usize, usize, W); V]`.
- Remove `IterVertices` for `Vec<T>`.
- Remove `IterVertices` for `[T; V]`.
- Remove `Outdegree` for `Vec<HashMap<usize, W>>`.
- Remove `Outdegree` for `Vec<HashSet<usize>>`.
- Remove `Outdegree` for `Vec<Vec<T>>`.
- Remove `Outdegree` for `[HashMap<usize, W>; V]>`.
- Remove `Outdegree` for `[HashSet<usize>; V]>`.
- Remove `Outdegree` for `[Vec<T>; V]`.
- Remove `RemoveArc` for `Vec<HashMap<usize, W>>`.
- Remove `RemoveArc` for `Vec<HashSet<usize>>`.
- Remove `RemoveArc` for `[HashMap<usize, W>; V]`.
- Remove `RemoveArc` for `[HashSet<usize>; V]`.

## [0.6.3] - 2024-04-06

Fix

- Fix `README` formatting.

## [0.6.2] - 2024-04-06

Added

- Add more tests to `algo::bfs`.
- Add more tests to `algo::dijkstra`.
- Implement `AddArc` for `[HashSet<usize>]`.
- Implement `AddArc` for `[Vec<usize>]`.
- Implement `AddWeightedArc` for `[HashMap<usize, W>]`.
- Implement `AddWeightedArc` for `[HashSet<(usize, W)>]`.
- Implement `AddWeightedArc` for `[Vec<(usize, W)>]`.
- Implement `IterAllArcs` for `[(usize, usize)]`.
- Implement `IterAllWeightedArcs` for `[(usize, usize, W)]`.

## [0.6.1] - 2024-04-06

Added

- Add "algorithms" and "mathematics" to `Cargo.toml` categories.
- Add "bfs" and "dijkstra" to `Cargo.toml` keywords.

Removed

- Remove redundant `homepage` metadata.

## [0.6.0] - 2024-04-06

Added

- Add `authors` to `README.md`.
- Add a doctest for `AdjacencyMatrix::new`.
- Add a doctest for `AdjacencyMatrix::toggle`.
- Add implementation documentation for `AddArc`.
- Add implementation documentation for `AddWeightedArc`.
- Add implementation documentation for `Size`.
- Add implementation documentation for `Order`.
- Add implementation documentation for `ArcWeight`.
- Add implementation documentation for `Indegree`.
- Add implementation documentation for `IsArc`.
- Add implementation documentation for `IterAllArcs`.
- Add implementation documentation for `IterAllWeightedArcs`.
- Add implementation documentation for `IterArcs`.
- Add implementation documentation for `IterWeightedArcs`.
- Add implementation documentation for `Outdegree`.
- Add implementation documentation for `RemoveArc`.

Changed

- Move `ops` to `op`.
- Adapt benchmark code to linting rules.
- Move the doctest trait properties to `op::prop`.
- Move the lints from `lib.rs` to `Cargo.toml`.

## [0.5.3] - 2024-04-05

Added

- Add a doctest for `op::add_weighted_arc::AddWeightedArc`.
- Add a doctest for `op::size::Size`.
- Add a doctest for `op::order::Order`.
- Add a doctest for `op::arc_weight::ArcWeight`.
- Add a doctest for `op::indegree::Indegree`.
- Add a doctest for `op::is_arc::IsArc`.
- Add a doctest for `op::iter_all_arcs::IterAllArcs`.
- Add a doctest for `op::iter_all_weighted_arcs::IterAllWeightedArcs`.
- Add a doctest for `op::iter_arcs::IterArcs`.
- Add a doctest for `op::iter_vertices::IterVertices`.
- Add a doctest for `op::iter_weighted_arcs::IterWeightedArcs`.
- Add a doctest for `op::outdegree::OutDegree`.
- Add a doctest for `op::remove_arc::RemoveArc`.
- Add documentation for `op::add_weighted_arc`.
- Add documentation for `op::size`.
- Add documentation for `op::order`.
- Add documentation for `op::arc_weight`.
- Add documentation for `op::indegree`.
- Add documentation for `op::is_arc`.
- Add documentation for `op::iter_all_arcs`.
- Add documentation for `op::iter_all_weighted_arcs`.
- Add documentation for `op::iter_arcs`.
- Add documentation for `op::iter_vertices`.
- Add documentation for `op::iter_weighted_arcs`.
- Add documentation for `op::outdegree`.
- Add documentation for `op::remove_arc`.

## [0.5.2] - 2024-04-04

Added

- Add doctest for `op::add_arc::AddArc`.
- Add documentation for `op::add_arc`.
- Add module-level doctest for `algo::bfs`.
- Add module-level doctest for `algo::dijkstra`.

## [0.5.1] - 2024-04-04

Added

- Add `bfs::predecessors_single_source`.
- Add `dijkstra::predecessors_single_source`.

## [0.5.0] - 2024-04-03

Added

- Implement `Indegree` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsArc` for `HashSet<(usize, usize)>`.
- Implement `IterVertices` for `HashSet<T>`.
- Implement `IterWeightedArcs` for `Vec<HashMap<usize, W>>`.
- Implement `IterWeightedArcs` for `[HashMap<usize, W>; V]`.
- Implement `Outdegree` for `Vec<HashMap<usize, W>>`.
- Implement `Outdegree` for `[HashMap<usize, W>; V]`.

Removed

- Remove the `VertexWeight` trait.

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
- Add a doctest example for `algo::dijkstra::unweighted::min_distances_single_source`
- Add a doctest example for `algo::dijkstra::unweighted::min_distances`
- Add a doctest example for `algo::dijkstra::weighted::min_distances_single_source`.

Changed

- Move `algo::DijkstraUnweighted::dijkstra` to `algo::dijkstra::unweighted::min_distances`.
- Move `algo::DijkstraWeighted::dijkstra` to `algo::dijkstra::weighted::min_distances`.
- Move `algo::dijkstra::dijkstra_sssp_weighted` to `algo::dijkstra::weighted::min_distances_single_source`.

Removed

- Remove the `new` benchmark.

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

- Enable selected lints from the `restriction` group.
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
