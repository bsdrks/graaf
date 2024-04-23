//! A trait to generate linear graphs
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Linear;
//!
//! //
//! assert_eq!(Vec::<Vec<usize>>::linear(0), Vec::<Vec<usize>>::new());
//!
//! // 0
//! assert_eq!(Vec::<Vec<usize>>::linear(1), vec![Vec::new()]);
//!
//! // 0 → 1
//! assert_eq!(Vec::<Vec<usize>>::linear(2), vec![vec![0, 1]]);
//!
//! // 0 → 1 → 2
//! assert_eq!(Vec::<Vec<usize>>::linear(3), vec![vec![0, 1], vec![1, 2]]);
//! ```

/// A trait to generate linear graphs
pub trait Linear {
    /// Generate a linear graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn linear(v: usize) -> Self;
}

impl Linear for Vec<Vec<usize>> {
    fn linear(v: usize) -> Self {
        match v {
            0 => Self::new(),
            1 => vec![Vec::new()],
            _ => (0..v - 1).map(|s| vec![s, s + 1]).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec() {
        for (v, g) in [
            //
            Vec::new(),
            // 0
            vec![Vec::new()],
            // 0 → 1
            vec![vec![0, 1]],
            // 0 → 1 → 2
            vec![vec![0, 1], vec![1, 2]],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::linear(v), g);
        }
    }
}
