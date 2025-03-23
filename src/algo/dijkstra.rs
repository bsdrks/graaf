//! Dijkstra's algorithm.
//!
//! Dijkstra's algorithm with binary heap finds the shortest path in an
//! arc-weighted digraph.[^1]
//!
//! The time complexity is `O(v log v + a)`, where `v` is the digraph's order
//! and `a` is the digraph's size.
//!
//! # Examples
//!
//! ## Single source
//!
//! The path from vertex `0` is red. `t` denotes the iteration indices.
//!
//! ![A digraph and the shortest path from source vertex `0` obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_1-0.87.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AddArcWeighted,
//!         AdjacencyListWeighted,
//!         Dijkstra,
//!         Empty,
//!     },
//!     std::iter::once,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
//!
//! digraph.add_arc_weighted(0, 1, 1);
//! digraph.add_arc_weighted(1, 2, 1);
//! digraph.add_arc_weighted(1, 6, 6);
//! digraph.add_arc_weighted(2, 4, 1);
//! digraph.add_arc_weighted(3, 0, 2);
//! digraph.add_arc_weighted(4, 5, 2);
//! digraph.add_arc_weighted(5, 6, 1);
//!
//! let mut dijkstra = Dijkstra::new(&digraph, once(0));
//!
//! assert!(dijkstra.eq([0, 1, 2, 4, 5, 6]));
//! ```
//!
//! ## Multiple sources
//!
//! The path from vertex `0` is red. The path from vertex `3` is blue.
//!
//! ![A digraph and the shortest path from two source vertices obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_multi_source_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     Dijkstra,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
//!
//! digraph.add_arc_weighted(0, 1, 1);
//! digraph.add_arc_weighted(1, 2, 1);
//! digraph.add_arc_weighted(1, 6, 5);
//! digraph.add_arc_weighted(2, 4, 1);
//! digraph.add_arc_weighted(3, 0, 2);
//! digraph.add_arc_weighted(3, 5, 1);
//! digraph.add_arc_weighted(4, 5, 1);
//! digraph.add_arc_weighted(5, 6, 3);
//!
//! let mut dijkstra = Dijkstra::new(&digraph, [0, 3].into_iter());
//!
//! assert!(dijkstra.eq([3, 0, 5, 1, 2, 4, 6]));
//! ```
//!
//! [^1]: Edsger Wybe Dijkstra. 1959. A note on two problems in connexion with
//!   graphs. Numer. Math. 1, 1 (December 1959), 269–271.
//!   <https://doi.org/10.1007/BF01386390>

use {
    crate::{
        Order,
        OutNeighborsWeighted,
    },
    core::cmp::Reverse,
    std::collections::BinaryHeap,
};

/// Dijkstra's algorithm.
///
/// # Examples
///
/// ## Single source
///
/// The path from vertex `0` is red. `t` denotes the iteration indices.
///
/// ![A digraph and the shortest path from source vertex `0` obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         AddArcWeighted,
///         AdjacencyListWeighted,
///         Dijkstra,
///         Empty,
///     },
///     std::iter::once,
/// };
///
/// let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
///
/// digraph.add_arc_weighted(0, 1, 1);
/// digraph.add_arc_weighted(1, 2, 1);
/// digraph.add_arc_weighted(1, 6, 6);
/// digraph.add_arc_weighted(2, 4, 1);
/// digraph.add_arc_weighted(3, 0, 2);
/// digraph.add_arc_weighted(4, 5, 2);
/// digraph.add_arc_weighted(5, 6, 1);
///
/// let mut dijkstra = Dijkstra::new(&digraph, once(0));
///
/// assert!(dijkstra.eq([0, 1, 2, 4, 5, 6]));
/// ```
///
/// ## Multiple sources
///
/// The path from vertex `0` is red. The path from vertex `3` is blue.
///
/// ![A digraph and the shortest path from two source vertices obtained by Dijkstra's algorithm](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/dijkstra_multi_source_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     Dijkstra,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyListWeighted::<usize>::empty(7);
///
/// digraph.add_arc_weighted(0, 1, 1);
/// digraph.add_arc_weighted(1, 2, 1);
/// digraph.add_arc_weighted(1, 6, 5);
/// digraph.add_arc_weighted(2, 4, 1);
/// digraph.add_arc_weighted(3, 0, 2);
/// digraph.add_arc_weighted(3, 5, 1);
/// digraph.add_arc_weighted(4, 5, 1);
/// digraph.add_arc_weighted(5, 6, 1);
///
/// let mut dijkstra = Dijkstra::new(&digraph, [0, 3].into_iter());
///
/// assert!(dijkstra.eq([3, 0, 5, 1, 6, 2, 4]));
/// ```
#[derive(Clone, Debug)]
pub struct Dijkstra<'a, D> {
    digraph: &'a D,
    dist: Vec<usize>,
    heap: BinaryHeap<(Reverse<usize>, usize)>,
}

impl<'a, D> Dijkstra<'a, D>
where
    D: Order,
{
    /// Initialize Dijkstra's algorithm.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `sources`: The source vertices.
    #[must_use]
    pub fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        D: Order,
        T: Iterator<Item = usize> + Clone,
    {
        let order = digraph.order();
        let mut dist = vec![usize::MAX; order];
        let mut heap = BinaryHeap::with_capacity(order);
        let dist_ptr = dist.as_mut_ptr();

        for u in sources {
            unsafe { *dist_ptr.add(u) = 0 };

            heap.push((Reverse(0), u));
        }

        Self {
            digraph,
            dist,
            heap,
        }
    }
}

impl<D> Iterator for Dijkstra<'_, D>
where
    D: Order + OutNeighborsWeighted<Weight = usize>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (Reverse(w_prev), u) = self.heap.pop()?;
        let dist_ptr = self.dist.as_mut_ptr();

        for (v, w) in self.digraph.out_neighbors_weighted(u) {
            let w_next = w + w_prev;
            let dist_v = unsafe { dist_ptr.add(v) };

            unsafe {
                if w_next < *dist_v {
                    *dist_v = w_next;

                    self.heap.push((Reverse(w_next), v));
                }
            }
        }

        if unsafe { *dist_ptr.add(u) } == w_prev {
            return Some(u);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::repr::adjacency_list_weighted::fixture::{
            bang_jensen_94_usize,
            bang_jensen_96_usize,
            kattis_bryr_1_usize,
            kattis_bryr_2_usize,
            kattis_bryr_3_usize,
            kattis_crosscountry_usize,
            kattis_shortestpath1_usize,
        },
        std::iter::once,
    };

    #[test]
    fn iter_bang_jensen_94() {
        let digraph = bang_jensen_94_usize();

        assert!(Dijkstra::new(&digraph, once(0)).eq([0, 2, 1, 5, 4, 3, 6]));
    }

    #[test]
    fn iter_bang_jensen_96() {
        let digraph = bang_jensen_96_usize();

        assert!(Dijkstra::new(&digraph, once(0)).eq([0, 2, 4, 1, 3, 5]));
    }

    #[test]
    fn iter_kattis_bryr_1() {
        let digraph = kattis_bryr_1_usize();

        assert!(Dijkstra::new(&digraph, once(0)).eq([0, 2, 1]));
    }

    #[test]
    fn iter_kattis_bryr_2() {
        let digraph = kattis_bryr_2_usize();

        assert!(Dijkstra::new(&digraph, once(0)).eq([0, 3, 1, 4, 2, 5]));
    }

    #[test]
    fn iter_kattis_bryr_3() {
        let digraph = kattis_bryr_3_usize();

        assert!(Dijkstra::new(&digraph, once(0))
            .eq([0, 3, 7, 5, 8, 4, 1, 9, 6, 2]));
    }

    #[test]
    fn iter_kattis_crosscountry() {
        let digraph = kattis_crosscountry_usize();

        assert!(Dijkstra::new(&digraph, once(0)).eq([0, 1, 2, 3]));
    }

    #[test]
    fn iter_kattis_shortestpath1() {
        let digraph = kattis_shortestpath1_usize();

        assert!(Dijkstra::new(&digraph, once(0)).eq([0, 1, 2]));
    }
}
