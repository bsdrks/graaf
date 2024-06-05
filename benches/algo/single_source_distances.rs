//! Benchmark different algorithms and types to calculate the distance from a
//! source vertex to all other vertices in a weighted digraph.

use graaf::algo::{
    bellman_ford_moore,
    fixture,
};

fn main() {
    divan::main();
}

#[divan::bench_group]
mod bang_jensen_94 {
    use super::*;

    #[divan::bench(sample_size = 1000)]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(
            &fixture::bang_jensen_94_weighted_isize(),
            0,
        );
    }

    #[divan::bench(sample_size = 1000)]
    fn bfs() {
        let _ = graaf::algo::bfs::single_source_distances(&fixture::bang_jensen_94(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(
            &fixture::bang_jensen_94_weighted_usize(),
            0,
        );
    }

    #[divan::bench(sample_size = 1000)]
    fn floyd_warshall() {
        let _ =
            graaf::algo::floyd_warshall::distances(&fixture::bang_jensen_94_weighted_isize())[0];
    }
}

#[divan::bench_group]
mod bang_jensen_96 {
    use super::*;

    #[divan::bench(sample_size = 1000)]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&fixture::bang_jensen_96_isize(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&fixture::bang_jensen_96(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&fixture::bang_jensen_96_isize())[0];
    }
}

#[divan::bench_group]
mod bang_jensen_99 {
    use super::*;

    #[divan::bench(sample_size = 1000)]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&fixture::bang_jensen_99(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&fixture::bang_jensen_99())[0];
    }
}

#[divan::bench_group]
mod kattis_bryr_1 {
    use super::*;

    #[divan::bench(sample_size = 1000)]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&fixture::kattis_bryr_1_isize(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&fixture::kattis_bryr_1(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&fixture::kattis_bryr_1_isize())[0];
    }
}

#[divan::bench_group]
mod kattis_bryr_2 {
    use super::*;

    #[divan::bench(sample_size = 1000)]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&fixture::kattis_bryr_2_isize(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&fixture::kattis_bryr_1(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&fixture::kattis_bryr_2_isize())[0];
    }
}

#[divan::bench_group]
mod kattis_bryr_3 {
    use super::*;

    #[divan::bench(sample_size = 1000)]
    fn bellman_ford_moore() {
        let _ = bellman_ford_moore::single_source_distances(&fixture::kattis_bryr_3_isize(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&fixture::kattis_bryr_1(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&fixture::kattis_bryr_3_isize())[0];
    }
}

#[divan::bench_group]
mod kattis_crosscountry {
    use super::*;

    #[divan::bench(sample_size = 1000)]
    fn bellman_ford_moore() {
        let _ =
            bellman_ford_moore::single_source_distances(&fixture::kattis_crosscountry_isize(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&fixture::kattis_crosscountry(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&fixture::kattis_crosscountry_isize())[0];
    }
}

#[divan::bench_group]
mod kattis_shortestpath1 {
    use super::*;

    #[divan::bench(sample_size = 1000)]
    fn bellman_ford_moore() {
        let _ =
            bellman_ford_moore::single_source_distances(&fixture::kattis_shortestpath1_isize(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn dijkstra() {
        let _ = graaf::algo::dijkstra::single_source_distances(&fixture::kattis_shortestpath1(), 0);
    }

    #[divan::bench(sample_size = 1000)]
    fn floyd_warshall() {
        let _ = graaf::algo::floyd_warshall::distances(&fixture::kattis_shortestpath1_isize())[0];
    }
}
