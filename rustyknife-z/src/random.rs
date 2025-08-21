use rand::{rngs::StdRng, FromEntropy, Rng, SeedableRng};
use std::ops::Range;

// Pcg32 = Lcg64Xsh32 has "16 bytes of state and 128-bit seeds", and is "considered value-stable
// (i.e. any change affecting the output given a fixed seed would be considered a breaking change
// to the crate)".
type RngImpl = rand_pcg::Lcg64Xsh32;

pub struct Random {
    rng: RngImpl,
    implicit: bool,
}

impl Random {
    pub fn new() -> Self {
        Random {
            rng: RngImpl::from_entropy(),
            implicit: true,
        }
    }

    pub fn from_rng(rng: &mut StdRng) -> Result<Self, std::io::Error> {
        let rng = RngImpl::from_rng(rng)?;
        Ok(Random {
            rng,
            implicit: false,
        })
    }

    pub fn get(&mut self, range: Range<u16>) -> u16 {
        self.rng.gen_range(range.start, range.end)
    }

    pub fn seed(&mut self, seed: u16) {
        self.rng = RngImpl::seed_from_u64(seed as u64);
    }

    pub fn seed_unpredictably(&mut self) {
        if self.implicit {
            self.rng = RngImpl::from_entropy();
        }
    }
}
