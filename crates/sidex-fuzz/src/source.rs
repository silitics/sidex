//! Deterministic choice source backing the generator.
//!
//! Every random decision the generator makes flows through the [`Source`] —
//! variant selection, list lengths, integer draws, character picks. The source
//! is seeded from a `u64` and backed by a ChaCha8 PRNG, so reproducing a value
//! requires only the same seed and IR. Each draw is recorded into a *choice
//! sequence*, which a future shrinker can reduce to minimize a counterexample.
//!
//! Smaller choice values map to "simpler" generated outputs. `draw_choice` 0
//! picks the first variant of a sum type; `draw_len` 0 yields an empty list;
//! `draw_int` near the lower bound is treated as the simplest integer. This
//! ordering is what shrinking depends on.

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// A deterministic, replayable source of random choices.
pub struct Source {
    rng: ChaCha8Rng,
    choices: Vec<u64>,
}

impl Source {
    /// Create a source seeded from `seed`.
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            choices: Vec::new(),
        }
    }

    /// The choice sequence recorded so far. The shrinker operates on this.
    pub fn choices(&self) -> &[u64] {
        &self.choices
    }

    /// Draw a uniform integer in `[lo, hi]` (inclusive on both ends).
    ///
    /// Smaller stored choices correspond to values closer to `lo`, so shrinking
    /// pulls integers toward their lower bound.
    pub fn draw_int(&mut self, lo: i64, hi: i64) -> i64 {
        debug_assert!(lo <= hi);
        let span = (hi as i128 - lo as i128) as u128 + 1;
        let raw = self.draw_u64();
        let offset = (raw as u128) % span;
        self.choices.push(offset as u64);
        lo + (offset as i64)
    }

    /// Draw a uniform unsigned integer in `[lo, hi]` (inclusive).
    pub fn draw_uint(&mut self, lo: u64, hi: u64) -> u64 {
        debug_assert!(lo <= hi);
        let span = (hi - lo) as u128 + 1;
        let raw = self.draw_u64();
        let offset = (raw as u128) % span;
        self.choices.push(offset as u64);
        lo + (offset as u64)
    }

    /// Draw a 64-bit float in `[0.0, 1.0)`.
    pub fn draw_f64_unit(&mut self) -> f64 {
        let raw = self.draw_u64();
        self.choices.push(raw);
        // 53-bit mantissa, divide by 2^53 to land in [0, 1).
        ((raw >> 11) as f64) * (1.0_f64 / ((1u64 << 53) as f64))
    }

    /// Draw a boolean. `false` is the simpler choice (stored as 0).
    pub fn draw_bool(&mut self) -> bool {
        let raw = self.draw_u64() & 1;
        self.choices.push(raw);
        raw == 1
    }

    /// Draw an index in `[0, n)`. Panics if `n == 0`.
    pub fn draw_choice(&mut self, n: usize) -> usize {
        assert!(n > 0, "draw_choice requires a non-empty range");
        let raw = self.draw_u64();
        let idx = (raw as u128 % n as u128) as usize;
        self.choices.push(idx as u64);
        idx
    }

    /// Draw a length in `[0, max]`, biased toward small values.
    ///
    /// The bias is exponential: each successive length is half as likely as the
    /// previous, so the expected length is `~1` regardless of `max`. This keeps
    /// generated structures small without hard-capping them, and gives shrinking
    /// a clear "smaller is simpler" signal.
    pub fn draw_len(&mut self, max: usize) -> usize {
        if max == 0 {
            self.choices.push(0);
            return 0;
        }
        let mut len = 0;
        while len < max && self.draw_bool() {
            len += 1;
        }
        len
    }

    /// Draw `count` ASCII characters into a `String`.
    pub fn draw_ascii_string(&mut self, count: usize) -> String {
        let mut out = String::with_capacity(count);
        for _ in 0..count {
            // Printable ASCII: 0x20..=0x7E.
            let ch = self.draw_uint(0x20, 0x7E) as u8 as char;
            out.push(ch);
        }
        out
    }

    fn draw_u64(&mut self) -> u64 {
        use rand::RngCore;
        self.rng.next_u64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Source::from_seed(42);
        let mut b = Source::from_seed(42);
        for _ in 0..100 {
            assert_eq!(a.draw_int(0, 1000), b.draw_int(0, 1000));
        }
    }

    #[test]
    fn draw_choice_in_range() {
        let mut s = Source::from_seed(1);
        for _ in 0..1000 {
            let c = s.draw_choice(7);
            assert!(c < 7);
        }
    }

    #[test]
    fn records_choices() {
        let mut s = Source::from_seed(7);
        s.draw_int(10, 20);
        s.draw_bool();
        s.draw_choice(4);
        assert_eq!(s.choices().len(), 3);
    }
}
