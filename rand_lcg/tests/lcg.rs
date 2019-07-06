//
// lcg.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

use rand_core::{RngCore, SeedableRng};
use rand_lcg::ANSIC;

#[test]
fn test_construction() {
    let mut rng = ANSIC::seed_from_u64(0);
    rng.next_u32();
}
