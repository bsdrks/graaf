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

/// A general-purpose PRNG.
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
    /// Construct a new `Xoshiro256StarStar`.
    ///
    /// # Arguments
    ///
    /// * `seed`: The seed value.
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
        let mut split_mix64 = SplitMix64::new(seed);

        let state = [
            split_mix64.next().unwrap(),
            split_mix64.next().unwrap(),
            split_mix64.next().unwrap(),
            split_mix64.next().unwrap(),
        ];

        Self { state }
    }

    /// Generate a pseudo-random boolean.
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

    /// Generate a pseudo-random `f64` in the range `[0, 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::gen::prng::Xoshiro256StarStar;
    ///
    /// let mut rng = Xoshiro256StarStar::new(123);
    /// let x = rng.next_f64();
    ///
    /// assert!(x >= 0.0 && x < 1.0);
    ///
    /// let y = rng.next_f64();
    ///
    /// assert!(y >= 0.0 && y < 1.0);
    /// ```
    ///
    /// # Panics
    ///
    /// This function never panics.
    #[must_use]
    pub fn next_f64(&mut self) -> f64 {
        const MANTISSA_BITS: u64 = 1 << 52;
        const MANTISSA_MASK: u64 = MANTISSA_BITS - 1;

        let next_u64 = self.next().unwrap();
        let mantissa = next_u64 & MANTISSA_MASK;
        let exponent = 1023; // Exponent for 2^0 in IEEE 754

        f64::from_bits((exponent << 52) | mantissa) - 1.0
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        proptest::*,
    };

    proptest! {
        #[test]
        fn next_f64(seed in 0..u64::MAX) {
            let mut rng = Xoshiro256StarStar::new(seed);

            prop_assert!((0.0..1.0).contains(&rng.next_f64()));
        }
    }
}
