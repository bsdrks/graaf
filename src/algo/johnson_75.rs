//! Johnson's circuit-finding algorithm.
//!
//! # Example
//!
//! ```
//! use graaf::{
//!     johnson_75::Johnson75,
//!     AdjacencyMap,
//!     Cycle,
//! };
//!
//! let digraph = AdjacencyMap::cycle(5);
//!
//! assert!(Johnson75::circuits(&digraph).eq(&[
//!     vec![0, 1],
//!     vec![0, 1, 2, 3, 4],
//!     vec![0, 4],
//!     vec![0, 4, 3, 2, 1],
//!     vec![1, 2],
//!     vec![2, 3],
//!     vec![3, 4]
//! ]));
//! ```

use {
    crate::{
        tarjan::strongly_connected_components,
        FilterVertices,
        Order,
        OutNeighbors,
        Vertices,
    },
    std::collections::BTreeSet,
};

/// Johnson's circuit-finding algorithm.
#[derive(Debug)]
pub struct Johnson75<'a, D> {
    a: &'a D,
    b: Vec<BTreeSet<usize>>,
    blocked: BTreeSet<usize>,
    stack: Vec<usize>,
}

impl<'a, D> Johnson75<'a, D>
where
    D: FilterVertices + Order + OutNeighbors + Vertices,
{
    fn is_blocked(&self, u: usize) -> bool {
        self.blocked.contains(&u)
    }

    fn unblock(&mut self, u: usize) {
        if self.is_blocked(u) {
            let _ = self.blocked.remove(&u);

            while let Some(v) = self.b[u].pop_first() {
                self.unblock(v);
            }
        }
    }

    fn circuit(
        &mut self,
        v: usize,
        s: usize,
        scc: &D,
        result: &mut Vec<Vec<usize>>,
    ) -> bool {
        self.stack.push(v);

        let mut f = false;
        let _ = self.blocked.insert(v);

        for w in scc.out_neighbors(v) {
            if w == s {
                result.push(self.stack.clone());

                f = true;
            } else if !self.is_blocked(w) && self.circuit(w, s, scc, result) {
                f = true;
            }
        }

        if f {
            self.unblock(v);
        } else {
            for w in scc.out_neighbors(v) {
                let _ = self.b[w].insert(v);
            }
        }

        let _ = self.stack.pop();

        f
    }

    /// Find circuits.
    ///
    /// # Panics
    ///
    /// Panics if there is a bug in `strongly_connected_components`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     johnson_75::Johnson75,
    ///     AdjacencyMap,
    ///     Cycle,
    /// };
    ///
    /// let digraph = AdjacencyMap::cycle(5);
    ///
    /// assert!(Johnson75::circuits(&digraph).eq(&[
    ///     vec![0, 1],
    ///     vec![0, 1, 2, 3, 4],
    ///     vec![0, 4],
    ///     vec![0, 4, 3, 2, 1],
    ///     vec![1, 2],
    ///     vec![2, 3],
    ///     vec![3, 4]
    /// ]));
    /// ```
    pub fn circuits(a: &'a D) -> Vec<Vec<usize>> {
        let state: &mut Johnson75<'a, D> = &mut Self {
            a,
            b: vec![BTreeSet::new(); a.order()],
            blocked: BTreeSet::new(),
            stack: Vec::new(),
        };

        let mut result = Vec::new();

        for s in state.a.vertices() {
            let subgraph = state.a.filter_vertices(|u| u >= s);
            let sccs = strongly_connected_components(&subgraph);

            if let Some(min_scc) =
                sccs.iter().min_by_key(|scc| scc.iter().min())
            {
                let scc = state.a.filter_vertices(|u| min_scc.contains(&u));

                if scc.order() > 0 {
                    let &start = min_scc.iter().min().unwrap();

                    for vertex in scc.vertices() {
                        let _ = state.blocked.remove(&vertex);

                        state.b[vertex].clear();
                    }

                    let _ = state.circuit(start, start, &scc, &mut result);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            AdjacencyMap,
            Biclique,
            Circuit,
            Cycle,
        },
    };

    #[test]
    fn biclique_2_2() {
        let digraph = AdjacencyMap::biclique(2, 2);

        assert!(Johnson75::circuits(&digraph).eq(&[
            vec![0, 2],
            vec![0, 2, 1, 3],
            vec![0, 3],
            vec![0, 3, 1, 2],
            vec![1, 2],
            vec![1, 3]
        ]));
    }

    #[test]
    fn biclique_2_3() {
        let digraph = AdjacencyMap::biclique(2, 3);

        assert!(Johnson75::circuits(&digraph).eq(&[
            vec![0, 2],
            vec![0, 2, 1, 3],
            vec![0, 2, 1, 4],
            vec![0, 3],
            vec![0, 3, 1, 2],
            vec![0, 3, 1, 4],
            vec![0, 4],
            vec![0, 4, 1, 2],
            vec![0, 4, 1, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4]
        ]));
    }

    #[test]
    fn circuit_3() {
        let digraph = AdjacencyMap::circuit(3);

        assert!(Johnson75::circuits(&digraph).eq(&[vec![0, 1, 2]]));
    }

    #[test]
    fn circuit_4() {
        let digraph = AdjacencyMap::circuit(4);

        assert!(Johnson75::circuits(&digraph).eq(&[vec![0, 1, 2, 3]]));
    }

    #[test]
    fn circuit_5() {
        let digraph = AdjacencyMap::circuit(5);

        assert!(Johnson75::circuits(&digraph).eq(&[vec![0, 1, 2, 3, 4]]));
    }

    #[test]
    fn cycle_3() {
        let digraph = AdjacencyMap::cycle(3);

        assert!(Johnson75::circuits(&digraph).eq(&[
            vec![0, 1],
            vec![0, 1, 2],
            vec![0, 2],
            vec![0, 2, 1],
            vec![1, 2]
        ]));
    }

    #[test]
    fn cycle_4() {
        let digraph = AdjacencyMap::cycle(4);

        assert!(Johnson75::circuits(&digraph).eq(&[
            vec![0, 1],
            vec![0, 1, 2, 3],
            vec![0, 3],
            vec![0, 3, 2, 1],
            vec![1, 2],
            vec![2, 3]
        ]));
    }

    #[test]
    fn cycle_5() {
        let digraph = AdjacencyMap::cycle(5);

        assert!(Johnson75::circuits(&digraph).eq(&[
            vec![0, 1],
            vec![0, 1, 2, 3, 4],
            vec![0, 4],
            vec![0, 4, 3, 2, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4]
        ]));
    }
}
