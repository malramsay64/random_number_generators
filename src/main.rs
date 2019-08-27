//
// main.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

use itertools::Itertools;
use rand::prelude::*;
use rand_lcg::{ANSIC, CPP, RANDU};
use rand_pcg::Pcg32;
use rand_xorshift::XorShiftRng;
use rand_xoshiro::Xoroshiro128StarStar;
use sfmt::SFMT;

use clap::{arg_enum, value_t, App, Arg};

arg_enum! {
    #[derive(PartialEq, Debug)]
    enum Generators {
        ANSIC,
        RANDU,
        CPP,
        TWISTER,
        PCG,
        XORSHIFT,
        XOSHIRO,
    }
}

fn generate_numbers<R: Rng + ?Sized>(rng: &mut R, samples: u64, dimensions: u64) {
    for _ in 0..samples {
        println!("{}", (0..dimensions).map(|_| rng.gen::<u64>()).format(", "));
    }
}

fn main() {
    let matches = App::new("Random Number Generator")
        .version("0.1")
        .author("Malcolm Ramsay <malramsay64@gmail.com>")
        .about("Generates random numbers using a range of different algorithms")
        .arg(
            Arg::with_name("rng")
                .possible_values(&Generators::variants())
                .case_insensitive(true)
                .required(true),
        )
        .arg(
            Arg::with_name("dimensions")
                .short("d")
                .long("dimensions")
                .default_value("1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num_samples")
                .short("n")
                .long("num_samples")
                .default_value("100")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .takes_value(true),
        )
        .get_matches();

    let seed = match value_t!(matches, "num_samples", u64) {
        Ok(s) => s,
        Err(_) => rand::random::<u64>(),
    };

    let samples = value_t!(matches, "num_samples", u64).unwrap();
    let dimensions = value_t!(matches, "dimensions", u64).unwrap();
    let gen = value_t!(matches.value_of("rng"), Generators).unwrap_or_else(|e| e.exit());
    println!("{:?}", gen);
    match gen {
        Generators::ANSIC => generate_numbers(&mut ANSIC::seed_from_u64(seed), samples, dimensions),
        Generators::RANDU => generate_numbers(&mut RANDU::seed_from_u64(seed), samples, dimensions),
        Generators::CPP => generate_numbers(&mut CPP::seed_from_u64(seed), samples, dimensions),
        Generators::TWISTER => {
            generate_numbers(&mut SFMT::seed_from_u64(seed), samples, dimensions)
        }
        Generators::PCG => generate_numbers(&mut Pcg32::seed_from_u64(seed), samples, dimensions),
        Generators::XORSHIFT => {
            generate_numbers(&mut XorShiftRng::seed_from_u64(seed), samples, dimensions)
        }
        Generators::XOSHIRO => generate_numbers(
            &mut Xoroshiro128StarStar::seed_from_u64(seed),
            samples,
            dimensions,
        ),
    };
}
