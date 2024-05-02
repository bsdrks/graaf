//! A trait to get the target vertex of an adjacency list edge
//!
//! # Examples
//!
//! ```
//! use graaf::op::Target;
//!
//! assert_eq!(0.target(), 0);
//! assert_eq!((3, 2).target(), 3);
//! assert_eq!((3, -2).target(), 3);
//! ```

/// A trait to get the target vertex of an adjacency list edge
///
/// # Examples
///
/// ```
/// use graaf::op::Target;
///
/// assert_eq!(0.target(), 0);
/// assert_eq!((3, 0).target(), 3);
/// assert_eq!((3, -2).target(), 3);
/// ```
pub trait Target {
    /// Returns the target vertex.
    fn target(&self) -> usize;
}

impl Target for usize {
    fn target(&self) -> usize {
        *self
    }
}

impl<W> Target for (usize, W) {
    fn target(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize() {
        assert_eq!(0.target(), 0);
        assert_eq!(3.target(), 3);
    }

    #[test]
    fn tuple() {
        assert_eq!((0, 0).target(), 0);
        assert_eq!((3, 2).target(), 3);
        assert_eq!((3, -2).target(), 3);
    }
}
