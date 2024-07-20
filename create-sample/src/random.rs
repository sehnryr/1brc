//! Random number generator.
//!
//! Code taken from [fastrand](https://crates.io/crates/fastrand)

// Chosen by fair roll of the dice.
const DEFAULT_RNG_SEED: u64 = 0xef6f79ed30ba75a;

/// A random number generator.
pub struct Random {
    seed: u64,

    // Normal distribution state.
    u: f64,
    v: f64,
    phase: bool,
}

impl Default for Random {
    fn default() -> Self {
        Self::new(DEFAULT_RNG_SEED)
    }
}

impl Random {
    /// Creates a new random number generator with the given seed.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            u: 0.0,
            v: 0.0,
            phase: true,
        }
    }

    /// Generates a random `u64`.
    #[inline]
    pub fn gen_u64(&mut self) -> u64 {
        const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
        const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;

        let s = self.seed.wrapping_add(WY_CONST_0);
        self.seed = s;
        let t = u128::from(s) * u128::from(s ^ WY_CONST_1);

        (t as u64) ^ (t >> 64) as u64
    }

    /// Generates a random `f64` in range `0..1`.
    #[inline]
    pub fn gen_f64(&mut self) -> f64 {
        let b = 64;
        let f = f64::MANTISSA_DIGITS - 1;
        f64::from_bits((1 << (b - 2)) - (1 << f) + (self.gen_u64() >> (b - f))) - 1.0
    }

    /// Generates a normal distributed `f64` with the given mean and standard deviation.
    #[inline]
    pub fn gen_normal(&mut self, mean: f64, std_dev: f64) -> f64 {
        let z: f64 = if self.phase {
            self.u = self.gen_f64();
            self.v = self.gen_f64();
            (-2.0 * self.u.ln()).sqrt() * (2.0 * std::f64::consts::PI * self.v).sin()
        } else {
            (-2.0 * self.u.ln()).sqrt() * (2.0 * std::f64::consts::PI * self.v).cos()
        };

        self.phase = !self.phase;

        z * std_dev + mean
    }
}
