//
// main.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

use itertools::Itertools;
use rand::prelude::*;
use rand_lcg::{ANSIC, CPP, RANDU};
use rand_pcg::Pcg64;
use rand_xorshift::XorShiftRng;
use rand_xoshiro::Xoroshiro128StarStar;
use sfmt::SFMT;

use clap::arg_enum;
use structopt::StructOpt;

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

#[derive(Debug, StructOpt)]
#[structopt(name = "random_number_generators")]
struct Args {
    /// The psuedo random number generator to use
    #[structopt(possible_values = &Generators::variants(), case_insensitive = true)]
    generator: Generators,

    /// The number of columns of data to output
    #[structopt(short, long, default_value = "1")]
    dimensions: usize,

    /// The number of rows of data to generate
    #[structopt(short, default_value = "10")]
    num_samples: usize,

    /// A number with which to seed the initial state of the pseudo-random number generator.
    /// The default is to use system randomess to generate a state.
    #[structopt(short, long)]
    seed: Option<u64>,
}

/// Print randomly generated numbers to standard out
///
/// # Parameters
/// - `rng`: The random number generater used to generate the numbers
/// - `samples`: How many lines of output to generate
/// - `dimensions`: The number of points on each line to generate
///
fn generate_numbers<R: Rng + ?Sized>(rng: &mut R, samples: usize, dimensions: usize) {
    for _ in 0..samples {
        println!("{}", (0..dimensions).map(|_| rng.gen::<u32>()).format(", "));
    }
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let seed = match args.seed {
        Some(s) => s,
        // When there is no given seed value, generate a truly random value
        None => rand::rngs::OsRng.next_u64(),
    };

    // Print out the random number generator we have chosen
    println!("{:?}", args.generator);

    // Generate the random numbers using the selected random number generator.
    match args.generator {
        Generators::ANSIC => generate_numbers(
            &mut ANSIC::seed_from_u64(seed),
            args.num_samples,
            args.dimensions,
        ),
        Generators::RANDU => generate_numbers(
            &mut RANDU::seed_from_u64(seed),
            args.num_samples,
            args.dimensions,
        ),
        Generators::CPP => generate_numbers(
            &mut CPP::seed_from_u64(seed),
            args.num_samples,
            args.dimensions,
        ),
        Generators::TWISTER => generate_numbers(
            &mut SFMT::seed_from_u64(seed),
            args.num_samples,
            args.dimensions,
        ),
        Generators::PCG => generate_numbers(
            &mut Pcg64::seed_from_u64(seed),
            args.num_samples,
            args.dimensions,
        ),
        Generators::XORSHIFT => generate_numbers(
            &mut XorShiftRng::seed_from_u64(seed),
            args.num_samples,
            args.dimensions,
        ),
        Generators::XOSHIRO => generate_numbers(
            &mut Xoroshiro128StarStar::seed_from_u64(seed),
            args.num_samples,
            args.dimensions,
        ),
    };
    Ok(())
}
