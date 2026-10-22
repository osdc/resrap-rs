use std::time::{SystemTime, UNIX_EPOCH};

/// The xorshift PRNG used by the Go implementation.
///
/// A seed of `0` is treated as "pick a random seed", exactly like
/// `newPRNG(0)` in the reference package.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct PRNG {
    seed: u64,
    number: u64,
}

impl PRNG {
    pub fn new(seed: u64) -> Self {
        let mut prng = PRNG { seed: 0, number: 0 };
        if seed == 0 {
            prng.generate_seed();
        } else {
            prng.set_seed(seed);
        }
        prng
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.number = seed;
        self.seed = seed;
    }

    #[cfg(test)]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    fn generate_seed(&mut self) {
        // No external rand crate: mix the wall clock with an address-like
        // value so two instances created in quick succession still differ.
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        let counter = SEED_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut seed = nanos ^ counter.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        if seed == 0 {
            seed = 0xDEAD_BEEF_CAFE_F00D;
        }
        self.set_seed(seed);
    }

    pub fn next_prn(&mut self) -> u64 {
        // Using the XOR shift method for PRN generation
        self.number ^= self.number << 13;
        self.number ^= self.number >> 7;
        self.number ^= self.number << 17;
        self.number
    }

    /// Returns a float in `[0, 1)`.
    pub fn random(&mut self) -> f64 {
        // Take the next 53 random bits, matching math/rand.Float64.
        let v = self.next_prn() >> 11;
        v as f64 / (1u64 << 53) as f64
    }

    /// Returns an integer in `[low, high)`.
    pub fn random_int(&mut self, low: usize, high: usize) -> usize {
        if high <= low {
            return low;
        }
        let r = self.random();
        low + (r * (high - low) as f64) as usize
    }
}

static SEED_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_is_deterministic() {
        let mut a = PRNG::new(42);
        let mut b = PRNG::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_prn(), b.next_prn());
        }
    }

    #[test]
    fn zero_seed_is_randomised() {
        let a = PRNG::new(0);
        assert_ne!(a.seed(), 0);
        let b = PRNG::new(0);
        assert_ne!(b.seed(), 0);
    }

    #[test]
    fn random_is_in_range() {
        let mut p = PRNG::new(7);
        for _ in 0..10_000 {
            let v = p.random();
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn random_int_honours_low_and_high() {
        let mut p = PRNG::new(7);
        for _ in 0..10_000 {
            let v = p.random_int(3, 8);
            assert!((3..8).contains(&v));
        }
    }

    #[test]
    fn empty_range_returns_low() {
        let mut p = PRNG::new(7);
        assert_eq!(p.random_int(5, 5), 5);
        assert_eq!(p.random_int(5, 4), 5);
    }
}
