//! The Bellman-Ford-Moore algorithm.
//!
//! Find the shortest distances from a source vertex to all other vertices in
//! an arc-weighted digraph with negative weights.
//!
//! Runs in **O(va)** time (worst-case), where **v** is the number of vertices
//! and **a** is the number of arcs.
//!
//! # Examples
//!
//! ## Shortest distances
//!
//! The shortest path from vertex `0` to `4` is red. The dashed arcs represent
//! the other shortest distances.
//!
//! ![A digraph and the shortest distances between the source vertex and the other vertices](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_1-0.87.4.svg?)
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     BellmanFordMoore,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<isize>::empty(6);
//!
//! digraph.add_arc_weighted(0, 1, 8);
//! digraph.add_arc_weighted(0, 2, 4);
//! digraph.add_arc_weighted(1, 2, -5);
//! digraph.add_arc_weighted(2, 3, -2);
//! digraph.add_arc_weighted(2, 4, 4);
//! digraph.add_arc_weighted(3, 5, -2);
//! digraph.add_arc_weighted(4, 3, 10);
//! digraph.add_arc_weighted(4, 5, 9);
//! digraph.add_arc_weighted(5, 3, 5);
//! digraph.add_arc_weighted(5, 4, -3);
//!
//! let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
//! let dist = bellman_ford_moore.distances();
//!
//! assert!(dist.unwrap().eq(&[0, 8, 3, 1, -4, -1]));
//! ```
//!
//! ## Negative cycle
//!
//! There is no shortest path between vertices `0` and the other vertices due
//! to the negative cycle.
//!
//! ![A digraph with a negative cycle](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_2-0.87.4.svg)
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     BellmanFordMoore,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, -2);
//! digraph.add_arc_weighted(1, 2, -1);
//! digraph.add_arc_weighted(2, 0, -1);
//!
//! assert_eq!(BellmanFordMoore::new(&digraph, 0).distances(), None);
//! ```
#![doc(alias = "bellman_ford")]

use crate::{
    ArcsWeighted,
    Order,
};

/// Find the shortest distances from a source vertex to all other vertices in
/// an arc-weighted digraph with negative weights.
///
/// Runs in **O(va)** time (worst-case), where **v** is the number of vertices
/// and **a** is the number of arcs.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `s`: The source vertex.
///
/// # Returns
///
/// The distances from the source vertex to all other vertices. Returns `None`
/// if the digraph contains a negative circuit.
///
/// # Examples
///
/// ## Shortest distances
///
/// The shortest path from vertex `0` to `4` is red. The dashed arcs represent
/// the other shortest distances.
///
/// ![A digraph and the shortest distances between the source vertex and the other vertices](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_1-0.87.4.svg?)
///
/// ```
/// use graaf::{
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     BellmanFordMoore,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyListWeighted::<isize>::empty(6);
///
/// digraph.add_arc_weighted(0, 1, 8);
/// digraph.add_arc_weighted(0, 2, 4);
/// digraph.add_arc_weighted(1, 2, -5);
/// digraph.add_arc_weighted(2, 3, -2);
/// digraph.add_arc_weighted(2, 4, 4);
/// digraph.add_arc_weighted(3, 5, -2);
/// digraph.add_arc_weighted(4, 3, 10);
/// digraph.add_arc_weighted(4, 5, 9);
/// digraph.add_arc_weighted(5, 3, 5);
/// digraph.add_arc_weighted(5, 4, -3);
///
/// let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
/// let dist = bellman_ford_moore.distances();
///
/// assert!(dist.unwrap().eq(&[0, 8, 3, 1, -4, -1]));
/// ```
///
/// ## Negative cycle
///
/// There is no shortest path between vertices `0` and the other vertices due
/// to the negative cycle.
///
/// ![A digraph with a negative cycle](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_2-0.87.4.svg)
///
/// ```
/// use graaf::{
///     AddArcWeighted,
///     AdjacencyListWeighted,
///     BellmanFordMoore,
///     Empty,
/// };
///
/// let mut digraph = AdjacencyListWeighted::empty(3);
///
/// digraph.add_arc_weighted(0, 1, -2);
/// digraph.add_arc_weighted(1, 2, -1);
/// digraph.add_arc_weighted(2, 0, -1);
///
/// assert_eq!(BellmanFordMoore::new(&digraph, 0).distances(), None);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BellmanFordMoore<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

impl<'a, D> BellmanFordMoore<'a, D> {
    /// Construct a new Bellman-Ford-Moore algorithm.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    /// * `s`: The source vertex.
    #[must_use]
    pub fn new(digraph: &'a D, s: usize) -> Self
    where
        D: Order,
    {
        let order = digraph.order();

        Self {
            digraph,
            dist: (0..order)
                .map(|u| if u == s { 0 } else { isize::MAX })
                .collect(),
        }
    }

    /// Find the shortest distances from a source vertex to all other vertices
    /// in an arc-weighted digraph with negative weights.
    ///
    /// Runs in **O(va)** time (worst-case), where **v** is the number of
    /// vertices and **a** is the number of arcs.
    ///
    /// # Returns
    ///
    /// The distances from the source vertex to all other vertices. Returns
    /// `None` if the digraph contains a negative circuit.
    ///
    /// # Examples
    ///
    /// ## Shortest distances
    ///
    /// The shortest path from vertex `0` to `4` is red. The dashed arcs
    /// represent the other shortest distances.
    ///
    /// ![A digraph and the shortest distances between the source vertex and the other vertices](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_1-0.87.4.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     BellmanFordMoore,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::<isize>::empty(6);
    ///
    /// digraph.add_arc_weighted(0, 1, 8);
    /// digraph.add_arc_weighted(0, 2, 4);
    /// digraph.add_arc_weighted(1, 2, -5);
    /// digraph.add_arc_weighted(2, 3, -2);
    /// digraph.add_arc_weighted(2, 4, 4);
    /// digraph.add_arc_weighted(3, 5, -2);
    /// digraph.add_arc_weighted(4, 3, 10);
    /// digraph.add_arc_weighted(4, 5, 9);
    /// digraph.add_arc_weighted(5, 3, 5);
    /// digraph.add_arc_weighted(5, 4, -3);
    ///
    /// let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
    /// let dist = bellman_ford_moore.distances();
    ///
    /// assert!(dist.unwrap().eq(&[0, 8, 3, 1, -4, -1]));
    /// ```
    ///
    /// ## Negative cycle
    ///
    /// There is no shortest path between vertices `0` and the other vertices
    /// due to the negative cycle.
    ///
    /// ![A digraph with a negative cycle](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bellman_ford_moore_2-0.87.4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     BellmanFordMoore,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::empty(3);
    ///
    /// digraph.add_arc_weighted(0, 1, -2);
    /// digraph.add_arc_weighted(1, 2, -1);
    /// digraph.add_arc_weighted(2, 0, -1);
    ///
    /// let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
    /// let dist = bellman_ford_moore.distances();
    ///
    /// assert_eq!(dist, None);
    /// ```
    #[must_use]
    pub fn distances(&mut self) -> Option<&[isize]>
    where
        D: ArcsWeighted<Weight = isize> + Order,
    {
        for _ in 1..self.digraph.order() {
            let mut updated = false;

            for (u, v, &w) in self.digraph.arcs_weighted() {
                let u = self.dist[u];

                if u == isize::MAX {
                    continue;
                }

                let w = u + w;

                if self.dist[v] > w {
                    self.dist[v] = w;

                    updated = true;
                }
            }

            if !updated {
                break;
            }
        }

        self.digraph
            .arcs_weighted()
            .all(|(u, v, &w)| {
                let u = self.dist[u];

                u == isize::MAX || self.dist[v] <= u + w
            })
            .then_some(&self.dist)
            .map(|v| &**v)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            repr::adjacency_list_weighted::fixture::{
                bang_jensen_94_isize,
                bang_jensen_96_isize,
                bang_jensen_99,
                kattis_bryr_1_isize,
                kattis_bryr_2_isize,
                kattis_bryr_3_isize,
                kattis_crosscountry_isize,
                kattis_shortestpath1_isize,
                kattis_shortestpath3,
            },
            AddArcWeighted,
            AdjacencyListWeighted,
            Empty,
        },
    };

    #[test]
    fn distances_trivial() {
        assert!(BellmanFordMoore::new(
            &AdjacencyListWeighted::<isize>::trivial(),
            0
        )
        .distances()
        .unwrap()
        .iter()
        .eq(&[0]));
    }

    #[test]
    fn distances_bang_jensen_94_weighted_0() {
        assert!(BellmanFordMoore::new(&bang_jensen_94_isize(), 0)
            .distances()
            .unwrap()
            .eq(&[0, 1, 1, 2, 2, 2, 3]));
    }

    #[test]
    fn distances_bang_jensen_94_weighted_1() {
        assert!(BellmanFordMoore::new(&bang_jensen_94_isize(), 1)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, 0, isize::MAX, 1, isize::MAX, 2, isize::MAX]));
    }

    #[test]
    fn distances_bang_jensen_94_weighted_2() {
        assert!(BellmanFordMoore::new(&bang_jensen_94_isize(), 2)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, 1, 0, 1, 1, 1, 2]));
    }

    #[test]
    fn distances_bang_jensen_94_weighted_3() {
        assert!(BellmanFordMoore::new(&bang_jensen_94_isize(), 3)
            .distances()
            .unwrap()
            .eq(&[
                isize::MAX,
                isize::MAX,
                isize::MAX,
                0,
                isize::MAX,
                1,
                isize::MAX
            ]));
    }

    #[test]
    fn distances_bang_jensen_94_weighted_4() {
        assert!(BellmanFordMoore::new(&bang_jensen_94_isize(), 4)
            .distances()
            .unwrap()
            .eq(&[
                isize::MAX,
                isize::MAX,
                isize::MAX,
                isize::MAX,
                0,
                isize::MAX,
                1
            ]));
    }

    #[test]
    fn distances_bang_jensen_94_weighted_5() {
        assert!(BellmanFordMoore::new(&bang_jensen_94_isize(), 5)
            .distances()
            .unwrap()
            .eq(&[
                isize::MAX,
                isize::MAX,
                isize::MAX,
                isize::MAX,
                isize::MAX,
                0,
                isize::MAX
            ]));
    }

    #[test]
    fn distances_bang_jensen_94_weighted_6() {
        assert!(BellmanFordMoore::new(&bang_jensen_94_isize(), 6)
            .distances()
            .unwrap()
            .eq(&[
                isize::MAX,
                isize::MAX,
                isize::MAX,
                isize::MAX,
                isize::MAX,
                isize::MAX,
                0
            ]));
    }

    #[test]
    fn distances_bang_jensen_96_0() {
        assert!(BellmanFordMoore::new(&bang_jensen_96_isize(), 0)
            .distances()
            .unwrap()
            .eq(&[0, 5, 3, 6, 4, 7]));
    }

    #[test]
    fn distances_bang_jensen_96_1() {
        assert!(BellmanFordMoore::new(&bang_jensen_96_isize(), 1)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, 0, 6, 2, 7, 3]));
    }

    #[test]
    fn distances_bang_jensen_96_2() {
        assert!(BellmanFordMoore::new(&bang_jensen_96_isize(), 2)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, 2, 0, 3, 1, 4]));
    }

    #[test]
    fn distances_bang_jensen_96_3() {
        assert!(BellmanFordMoore::new(&bang_jensen_96_isize(), 3)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, isize::MAX, isize::MAX, 0, isize::MAX, 1]));
    }

    #[test]
    fn distances_bang_jensen_96_4() {
        assert!(BellmanFordMoore::new(&bang_jensen_96_isize(), 4)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, 4, 2, 2, 0, 3]));
    }

    #[test]
    fn distances_bang_jensen_96_5() {
        assert!(BellmanFordMoore::new(&bang_jensen_96_isize(), 5)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, isize::MAX, isize::MAX, 2, isize::MAX, 0]));
    }

    #[test]
    fn distances_bang_jensen_99_0() {
        assert!(BellmanFordMoore::new(&bang_jensen_99(), 0)
            .distances()
            .unwrap()
            .eq(&[0, 8, 3, 1, -4, -1]));
    }

    #[test]
    fn distances_bang_jensen_99_1() {
        assert!(BellmanFordMoore::new(&bang_jensen_99(), 1)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, 0, -5, -7, -12, -9]));
    }

    #[test]
    fn distances_bang_jensen_99_2() {
        assert!(BellmanFordMoore::new(&bang_jensen_99(), 2)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, isize::MAX, 0, -2, -7, -4]));
    }

    #[test]
    fn distances_bang_jensen_99_3() {
        assert!(BellmanFordMoore::new(&bang_jensen_99(), 3)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, isize::MAX, isize::MAX, 0, -5, -2]));
    }

    #[test]
    fn distances_bang_jensen_99_4() {
        assert!(BellmanFordMoore::new(&bang_jensen_99(), 4)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, isize::MAX, isize::MAX, 10, 0, 8]));
    }

    #[test]
    fn distances_bang_jensen_99_5() {
        assert!(BellmanFordMoore::new(&bang_jensen_99(), 5)
            .distances()
            .unwrap()
            .eq(&[isize::MAX, isize::MAX, isize::MAX, 5, -3, 0]));
    }

    #[test]
    fn distances_kattis_bryr_1() {
        assert!(BellmanFordMoore::new(&kattis_bryr_1_isize(), 0)
            .distances()
            .unwrap()
            .eq(&[0, 1, 1]));
    }

    #[test]
    fn distances_kattis_bryr_2() {
        assert!(BellmanFordMoore::new(&kattis_bryr_2_isize(), 0)
            .distances()
            .unwrap()
            .eq(&[0, 1, 2, 1, 2, 3]));
    }

    #[test]
    fn distances_kattis_bryr_3() {
        assert!(BellmanFordMoore::new(&kattis_bryr_3_isize(), 0)
            .distances()
            .unwrap()
            .eq(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 1]));
    }

    #[test]
    fn distances_kattis_crosscountry() {
        assert!(BellmanFordMoore::new(&kattis_crosscountry_isize(), 0)
            .distances()
            .unwrap()
            .eq(&[0, 1, 3, 10]));
    }

    #[test]
    fn distances_kattis_shortestpath1() {
        assert!(BellmanFordMoore::new(&kattis_shortestpath1_isize(), 0)
            .distances()
            .unwrap()
            .eq(&[0, 2, 4, isize::MAX]));
    }

    #[test]
    fn distances_kattis_shortestpath3() {
        assert_eq!(
            BellmanFordMoore::new(&kattis_shortestpath3(), 0).distances(),
            None
        );
    }

    #[test]
    fn test_negative_circuit() {
        let mut digraph = AdjacencyListWeighted::<isize>::empty(3);

        digraph.add_arc_weighted(0, 1, -2);
        digraph.add_arc_weighted(1, 2, -1);
        digraph.add_arc_weighted(2, 0, -1);

        assert_eq!(BellmanFordMoore::new(&digraph, 0).distances(), None);
    }
}
