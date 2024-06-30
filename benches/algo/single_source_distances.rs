#![macro_use]
//! Benchmark different algorithms and types to calculate the distance from a
//! source vertex to all other vertices in a weighted digraph.

use graaf::algo::bellman_ford_moore;

fn main() {
    divan::main();
}

#[divan::bench_group(sample_size = 1000)]
mod bang_jensen_94 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&graaf::bang_jensen_94_weighted!(), 0);
    }

    #[divan::bench]
    fn bfs() {
        let _ = graaf::algo::bfs::single_source_distances(&graaf::bang_jensen_94!(), 0);
    }

    #[divan::bench]
    fn dijkstra() {
        let _ =
            graaf::algo::dijkstra::single_source_distances(&graaf::bang_jensen_94_weighted!(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&graaf::bang_jensen_94_weighted!())[0];
    }
}

#[divan::bench_group(sample_size = 1000)]
mod bang_jensen_96 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&graaf::bang_jensen_96!(), 0);
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&graaf::bang_jensen_96!(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&graaf::bang_jensen_96!())[0];
    }
}

#[divan::bench_group(sample_size = 1000)]
mod bang_jensen_99 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&graaf::bang_jensen_99!(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&graaf::bang_jensen_99!())[0];
    }
}

#[divan::bench_group(sample_size = 1000)]
mod kattis_bryr_1 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&graaf::kattis_bryr_1!(), 0);
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&graaf::kattis_bryr_1!(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&graaf::kattis_bryr_1!())[0];
    }
}

#[divan::bench_group(sample_size = 1000)]
mod kattis_bryr_2 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&graaf::kattis_bryr_2!(), 0);
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&graaf::kattis_bryr_1!(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&graaf::kattis_bryr_2!())[0];
    }
}

#[divan::bench_group(sample_size = 1000)]
mod kattis_bryr_3 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&graaf::kattis_bryr_3!(), 0);
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&graaf::kattis_bryr_1!(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&graaf::kattis_bryr_3!())[0];
    }
}

#[divan::bench_group(sample_size = 1000)]
mod kattis_crosscountry {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&graaf::kattis_crosscountry!(), 0);
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&graaf::kattis_crosscountry!(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&graaf::kattis_crosscountry!())[0];
    }
}

#[divan::bench_group(sample_size = 1000)]
mod kattis_shortestpath1 {
    use super::*;

    #[divan::bench]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&graaf::kattis_shortestpath1!(), 0);
    }

    #[divan::bench]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&graaf::kattis_shortestpath1!(), 0);
    }

    #[divan::bench]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&graaf::kattis_shortestpath1!())[0];
    }
}

#[divan::bench_group(sample_size = 1000)]
mod random_tournament {
    use {
        divan::Bencher,
        graaf::{
            adjacency_list,
            algo::bfs,
            gen::RandomTournament,
        },
    };

    #[divan::bench]
    fn bfs(bencher: Bencher<'_, '_>) {
        let digraph = adjacency_list::Digraph::random_tournament(100);

        bencher.bench_local(|| {
            let _ = bfs::single_source_distances(&digraph, 0);
        });
    }
}
