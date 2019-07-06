//
// lcg.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

use rand_core::{RngCore, SeedableRng};
use rand_lcg::{ANSIC, CPP, RANDU};

#[test]
fn test_seeded_construction_ansic() {
    let mut rng1 = ANSIC::seed_from_u64(0);
    let mut rng2 = ANSIC::seed_from_u64(0);

    for _ in 0..10 {
        let value1 = rng1.next_u32();
        let value2 = rng2.next_u32();

        assert_eq!(value1, value2);
    }

    for _ in 0..10 {
        let value1 = rng1.next_u64();
        let value2 = rng2.next_u64();

        assert_eq!(value1, value2);
    }
}

#[test]
fn test_seeded_construction_cpp() {
    let mut rng1 = CPP::seed_from_u64(0);
    let mut rng2 = CPP::seed_from_u64(0);

    for _ in 0..10 {
        let value1 = rng1.next_u32();
        let value2 = rng2.next_u32();

        assert_eq!(value1, value2);
    }

    for _ in 0..10 {
        let value1 = rng1.next_u64();
        let value2 = rng2.next_u64();

        assert_eq!(value1, value2);
    }
}

#[test]
fn test_seeded_construction_randu() {
    let mut rng1 = RANDU::seed_from_u64(0);
    let mut rng2 = RANDU::seed_from_u64(0);

    for _ in 0..10 {
        let value1 = rng1.next_u32();
        let value2 = rng2.next_u32();

        assert_eq!(value1, value2);
    }

    for _ in 0..10 {
        let value1 = rng1.next_u64();
        let value2 = rng2.next_u64();

        assert_eq!(value1, value2);
    }
}
