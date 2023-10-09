#[derive(Debug)]
pub struct Rng {
    pub seed: u64,
    pub exp_disabled: bool,
}

impl Rng {

    pub fn new(seed: u64) -> Self {
        Rng {
            seed,
            exp_disabled: false,
        }
    }

    pub fn next(&mut self) -> u64 {
        self.seed = (self.seed.wrapping_mul(6364136223846793005) + 1) & 0x7FFFFFFF_FFFFFFFF;
        self.seed
    }

    pub fn rand(&mut self, min: usize, max: usize) -> usize {
        assert!(max >= min, "Bad range specified for rand()");
        if min == max {
            return min;
        }
        if min == 0 && max == core::usize::MAX {
            return self.next() as usize;
        }
        let range = max - min + 1;
        let result = (self.next() % range as u64) as usize;
        result + min
    }

    pub fn rand_exp(&mut self, min: usize, max: usize) -> usize {
        if self.exp_disabled {
            return self.rand(min, max);
        }
        if self.rand(0, 1) == 0 {
            self.rand(min, max)
        } else {
            let x = self.rand(min, max);
            self.rand(min, x)
        }
    }
}
