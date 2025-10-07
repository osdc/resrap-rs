use rand::Rng;
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
    pub fn generate_seed(&mut self) {
        let mut rn = rand::rng();
        let var: u64 = rn.random();
        self.number = var;
        self.seed = var;
    }
    pub fn next_prn(&mut self) -> u64 {
        self.number ^= self.number << 13;
        self.number ^= self.number >> 7;
        self.number ^= self.number << 17;
        return self.number;
    }
    pub fn random(&mut self) -> f64 {
        let var = (self.next_prn() >> 11) as f64;
        var / ((1u64 << 53) - 1) as f64 // Divide by (2^53 - 1) to get [0, 1]
    }
    pub fn random_int(&mut self, min: i32, max: i32) -> i32 {
        if max < min {
            return min;
        }
        min + (max as f64 * self.random()) as i32
    }
}
