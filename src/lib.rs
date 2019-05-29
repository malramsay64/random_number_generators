//
// lib.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

use rand_core::{impls, le, Error, RngCore, SeedableRng};

#[macro_export]
macro_rules! LCG {
    ($name:ident, $modulus:expr, $multiplier:expr, $additive:expr) => {
        pub struct $name {
            state: u32,
        }

        impl RngCore for $name {
            #[inline]
            fn next_u32(&mut self) -> u32 {
                self.state = self.state.wrapping_mul($multiplier).wrapping_add($additive);
                self.state %= $modulus;
                self.state
            }

            #[inline]
            fn next_u64(&mut self) -> u64 {
                self.next_u32() as u64
            }

            #[inline]
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                impls::fill_bytes_via_next(self, dest)
            }

            #[inline]
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
                Ok(self.fill_bytes(dest))
            }
        }

        impl SeedableRng for $name {
            type Seed = [u8; 4];
            fn from_seed(seed: Self::Seed) -> Self {
                let mut state: [u32; 1] = [0];
                le::read_u32_into(&seed, &mut state);
                Self { state: state[0] }
            }
        }
    };
}

LCG!(RANDU, 1u32 << 31, 65539, 0);

LCG!(ANSIC, 1u32 << 31, 1_103_515_245, 12345);

LCG!(CPP, (1u32 << 31) - 1, 48271, 0);

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_invariant() {
        LCG!(TestZero, 2 ^ 31, 0, 0);
        let mut rng = TestZero::from_seed([0; 4]);
        for _ in 0..10 {
            assert!(rng.gen::<u64>() == 0);
        }
    }

    #[test]
    fn test_add() {
        LCG!(TestZero, 1 << 31, 1, 1);
        let mut rng = TestZero::from_seed([0; 4]);
        for i in 0..10 {
            let val: u32 = rng.gen();
            println!("Value: {} i: {}", val, i);
            assert!(val == i + 1);
        }
    }
}
