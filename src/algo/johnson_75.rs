//! Johnson's circuit-finding algorithm.

use {
    super::tarjan::strongly_connected_components,
    crate::{
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
    result: Vec<Vec<usize>>,
    stack: Vec<usize>,
}

impl<'a, D> Johnson75<'a, D>
where
    D: FilterVertices + Order + OutNeighbors + Vertices,
{
    /// Construct a new `Johnson75`.
    pub fn new(a: &'a D) -> Self {
        Self {
            a,
            b: vec![BTreeSet::new(); a.order()],
            blocked: BTreeSet::new(),
            result: Vec::new(),
            stack: Vec::new(),
        }
    }

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

    fn circuit(&mut self, v: usize, s: usize, scc: &D) -> bool {
        self.stack.push(v);

        let mut f = false;
        let _ = self.blocked.insert(v);

        for w in scc.out_neighbors(v) {
            if w == s {
                self.result.push(self.stack.clone());

                f = true;
            } else if !self.is_blocked(w) && self.circuit(w, s, scc) {
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
    pub fn find_circuits(&mut self) {
        for s in self.a.vertices() {
            let subgraph = self.a.filter_vertices(|u| u >= s);
            let sccs = strongly_connected_components(&subgraph);

            if let Some(min_scc) =
                sccs.iter().min_by_key(|scc| scc.iter().min())
            {
                let scc = self.a.filter_vertices(|u| min_scc.contains(&u));

                if scc.order() > 0 {
                    let &start = min_scc.iter().min().unwrap();

                    for vertex in scc.vertices() {
                        let _ = self.blocked.remove(&vertex);

                        self.b[vertex].clear();
                    }

                    let _ = self.circuit(start, start, &scc);
                }
            }
        }
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
    fn cycle_3() {
        let digraph = AdjacencyMap::cycle(3);
        let mut johnson = Johnson75::new(&digraph);

        johnson.find_circuits();

        assert!(johnson.result.eq(&[
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
        let mut johnson = Johnson75::new(&digraph);

        johnson.find_circuits();

        assert!(johnson.result.eq(&[
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
        let mut johnson = Johnson75::new(&digraph);

        johnson.find_circuits();

        assert!(johnson.result.eq(&[
            vec![0, 1],
            vec![0, 1, 2, 3, 4],
            vec![0, 4],
            vec![0, 4, 3, 2, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4]
        ]));
    }

    #[test]
    fn circuit_3() {
        let digraph = AdjacencyMap::circuit(3);
        let mut johnson = Johnson75::new(&digraph);

        johnson.find_circuits();

        assert!(johnson.result.eq(&[vec![0, 1, 2]]));
    }

    #[test]
    fn circuit_4() {
        let digraph = AdjacencyMap::circuit(4);
        let mut johnson = Johnson75::new(&digraph);

        johnson.find_circuits();

        assert!(johnson.result.eq(&[vec![0, 1, 2, 3]]));
    }

    #[test]
    fn circuit_5() {
        let digraph = AdjacencyMap::circuit(5);
        let mut johnson = Johnson75::new(&digraph);

        johnson.find_circuits();

        assert!(johnson.result.eq(&[vec![0, 1, 2, 3, 4]]));
    }

    #[test]
    fn biclique_2_2() {
        let digraph = AdjacencyMap::biclique(2, 2);
        let mut johnson = Johnson75::new(&digraph);

        johnson.find_circuits();

        assert!(johnson.result.eq(&[
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
        use crate::Arcs;

        let digraph = AdjacencyMap::biclique(2, 3);
        let mut johnson = Johnson75::new(&digraph);

        println!("{:?}", digraph.arcs().collect::<Vec<_>>());
        johnson.find_circuits();

        assert!(johnson.result.eq(&[
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
}
