use rand::{rngs::StdRng, Rng};
use rand_pcg::rand_core::RngCore;
use rand_pcg::rand_core::SeedableRng;
use std::ops::Range;

// Pcg32 = Lcg64Xsh32 has "16 bytes of state and 128-bit seeds", and is "considered value-stable
// (i.e. any change affecting the output given a fixed seed would be considered a breaking change
// to the crate)".
type RngImpl = rand_pcg::Lcg64Xsh32;

#[allow(dead_code)]
pub struct Random {
    rng: RngImpl,
    implicit: bool,
}

impl Random {
    #[cfg(feature = "os_rng")]
    pub fn new() -> Self {
        Random {
            rng: RngImpl::from_os_rng(),
            implicit: true,
        }
    }

    pub fn from_rng(rng: &mut StdRng) -> Self {
        let seed = rng.gen::<[u8; 16]>();
        let rng = RngImpl::from_seed(seed);
        Random {
            rng,
            implicit: false,
        }
    }

    pub fn get(&mut self, range: Range<u16>) -> u16 {
        // Sample uniformly in [start, end) (end exclusive) without modulo bias.
        let start = range.start as u32;
        let end = range.end as u32;
        let width = end.saturating_sub(start);
        if width == 0 {
            return range.start; // degenerate range
        }
        let zone = u32::MAX - (u32::MAX % width);
        loop {
            let x = self.rng.next_u32();
            if x < zone {
                return (start + (x % width)) as u16;
            }
        }
    }

    pub fn seed(&mut self, seed: u16) {
        self.rng = RngImpl::seed_from_u64(seed as u64);
    }

    pub fn seed_unpredictably(&mut self) {
        #[cfg(feature = "os_rng")]
        if self.implicit {
            self.rng = RngImpl::from_os_rng();
        }
    }
}
