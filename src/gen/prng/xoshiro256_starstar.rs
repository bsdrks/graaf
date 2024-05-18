//! A general-purpose PRNG
//!
//! # Examples
//!
//! ```
//! use graaf::gen::prng::Xoshiro256StarStar;
//!
//! let mut rng = Xoshiro256StarStar::new(123);
//!
//! assert_ne!(rng.next(), rng.next());
//! ```

use super::SplitMix64;

/// A general-purpose PRNG
///
/// # Examples
///
/// ```
/// use graaf::gen::prng::Xoshiro256StarStar;
///
/// let mut rng = Xoshiro256StarStar::new(123);
///
/// assert_ne!(rng.next(), rng.next());
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Xoshiro256StarStar {
    state: [u64; 4],
}

impl Xoshiro256StarStar {
    /// Constructs a new `Xoshiro256StarStar`
    ///
    /// # Arguments
    ///
    /// * `seed`: The seed value
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::gen::prng::Xoshiro256StarStar;
    ///
    /// let mut rng = Xoshiro256StarStar::new(123);
    ///
    /// assert_ne!(rng.next(), rng.next());
    /// ```
    ///
    /// # Panics
    ///
    /// This function never panics.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        let mut splitmix64 = SplitMix64::new(seed);

        let state = [
            splitmix64.next().unwrap(),
            splitmix64.next().unwrap(),
            splitmix64.next().unwrap(),
            splitmix64.next().unwrap(),
        ];

        Self { state }
    }

    /// Generates a pseudo-random boolean
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::gen::prng::Xoshiro256StarStar;
    ///
    /// let mut rng = Xoshiro256StarStar::new(123);
    ///
    /// // PRNGs are deterministic
    /// assert_eq!(rng.next_bool(), true);
    /// assert_eq!(rng.next_bool(), false);
    /// assert_eq!(rng.next_bool(), true);
    /// ```
    ///
    /// # Panics
    ///
    /// This function never panics.
    #[must_use]
    pub fn next_bool(&mut self) -> bool {
        self.next().unwrap() & 1 == 1
    }
}

impl Iterator for Xoshiro256StarStar {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        Some(s)
    }
}
