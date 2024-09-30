//! Johnson's circuit-finding algorithm.
//!
//! # Example
//!
//! ```
//! use graaf::{
//!     AdjacencyMap,
//!     Cycle,
//!     Johnson75,
//! };
//!
//! let digraph = AdjacencyMap::cycle(5);
//! let mut johnson = Johnson75::new(&digraph);
//!
//! assert!(johnson.circuits().eq(&[
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
        FilterVertices,
        Order,
        OutNeighbors,
        Tarjan,
        Vertices,
    },
    std::collections::BTreeSet,
};

/// Johnson's circuit-finding algorithm.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Johnson75<'a, D> {
    a: &'a D,
    b: Vec<BTreeSet<usize>>,
    blocked: BTreeSet<usize>,
    stack: Vec<usize>,
}

impl<'a, D> Johnson75<'a, D> {
    /// Construct a new instance of Johnson's circuit-finding algorithm.
    ///
    /// # Arguments
    ///
    /// * `a`: The digraph.
    #[must_use]
    pub fn new(a: &'a D) -> Self
    where
        D: Order,
    {
        Self {
            a,
            b: vec![BTreeSet::new(); a.order()],
            blocked: BTreeSet::new(),
            stack: Vec::new(),
        }
    }

    #[must_use]
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

    #[must_use]
    fn circuit(
        &mut self,
        v: usize,
        s: usize,
        scc: &D,
        result: &mut Vec<Vec<usize>>,
    ) -> bool
    where
        D: OutNeighbors,
    {
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
    /// Panics if there is a bug in [`Tarjan::components`].
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyMap,
    ///     Cycle,
    ///     Johnson75,
    /// };
    ///
    /// let digraph = AdjacencyMap::cycle(5);
    ///
    /// assert!(Johnson75::new(&digraph).circuits().eq(&[
    ///     vec![0, 1],
    ///     vec![0, 1, 2, 3, 4],
    ///     vec![0, 4],
    ///     vec![0, 4, 3, 2, 1],
    ///     vec![1, 2],
    ///     vec![2, 3],
    ///     vec![3, 4]
    /// ]));
    /// ```
    #[must_use]
    pub fn circuits(&mut self) -> Vec<Vec<usize>>
    where
        D: FilterVertices + Order + OutNeighbors + Vertices,
    {
        let mut result = Vec::new();

        for s in self.a.vertices() {
            let subgraph = self.a.filter_vertices(|u| u >= s);
            let mut tarjan = Tarjan::new(&subgraph);
            let components = tarjan.components();

            if let Some(min_scc) =
                components.iter().min_by_key(|scc| scc.iter().min())
            {
                let component =
                    self.a.filter_vertices(|u| min_scc.contains(&u));

                if component.order() > 0 {
                    let &start = min_scc.iter().min().unwrap();

                    for vertex in component.vertices() {
                        let _ = self.blocked.remove(&vertex);

                        self.b[vertex].clear();
                    }

                    let _ =
                        self.circuit(start, start, &component, &mut result);
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

        assert!(Johnson75::new(&digraph).circuits().eq(&[
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

        assert!(Johnson75::new(&digraph).circuits().eq(&[
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

        assert!(Johnson75::new(&digraph).circuits().eq(&[vec![0, 1, 2]]));
    }

    #[test]
    fn circuit_4() {
        let digraph = AdjacencyMap::circuit(4);

        assert!(Johnson75::new(&digraph).circuits().eq(&[vec![0, 1, 2, 3]]));
    }

    #[test]
    fn circuit_5() {
        let digraph = AdjacencyMap::circuit(5);

        assert!(Johnson75::new(&digraph)
            .circuits()
            .eq(&[vec![0, 1, 2, 3, 4]]));
    }

    #[test]
    fn cycle_3() {
        let digraph = AdjacencyMap::cycle(3);

        assert!(Johnson75::new(&digraph).circuits().eq(&[
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

        assert!(Johnson75::new(&digraph).circuits().eq(&[
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

        assert!(Johnson75::new(&digraph).circuits().eq(&[
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
