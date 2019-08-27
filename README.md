# Random Number Generators

This is a project demonstrating a range of random number generators
allowing a simple comparison of each.
This repository consists of two parts,
a command line utility
which allow the generation of pseudo random numbers
using a range of different algorithms.
The other component is a talk
about how we generate random numbers
on a computer.

## Installation of Command Line Utility

The command line utility is written in Rust,
which can be installed by following
the instructions at <https://rustup.rs/>.

Once the `rustup` tool has been installed
the command
```sh
cargo install random_number_generators
```
will download and install the binary
in the directory `~/.cargo/bin`

Alternatively,
instead of installing using cargo
the repository can be cloned
to your local computer using
```sh
git clone https://github.com/malramsay64/random_number_generators
```
which once inside the `random_number_generators` directory
```sh
cd random_number_generators
```
the command
```sh
cargo run -- --help
```
will build the binary for your machine,
and run it with the flag `--help`,
displaying all the options.

## The command line tool

The command line tool generates
sequences of random numbers
which are printed to the console (`stdout`).
There are a range of different Pseudo-Random Number Generators (PRNG)
from which to choose from

- ANSIC
- RANDU
- CPP
- Twister
- PCG
- Xorshift

Each of the different PRNG can be chosen
using the first argument.

As an example of use
I will generate 10,000 points in 3D space
using the RANDU generator
which I use to demonstrate how terrible it is.

```sh
random_number_generators randu --dimensions 3 --num_samples 10000 --seed 0
```

This can be saved to a text file for later analysis
using redirection

```sh
random_number_generators randu --dimensions 3 --num_samples 10000 --seed 0 > randu.txt
```

## The Presentation

A presentation covering
what PRNGs are,
some of their history
and a look at what we currently use.

The slides for the presentation are in the `slides.md` file
which is converted to latex using [pandoc]
and finally converted to a pdf using [tectonic].

### Installation of Tools

There are a number of tools used in this process
and to make it easier they are managed using the [conda] package manager.
This is available as a minimal image in [miniconda],
with installation instructions available [here][conda install].

Once conda is installed,
running the command

```sh
conda env create
```

in the `random_number_generators` directory
will install all the necessary packages
in a virtual environment
which can be accessed using the command

```sh
conda activate rng
```

With the environment activated,
the command

```sh
make
```

will create a file `main.pdf`
which contains the presentation.


[conda]: https://conda.io/en/latest/
[pandoc]: https://pandoc.org/
[tectonic]: https://tectonic-typesetting.github.io/en-US/
[miniconda]: https://docs.conda.io/en/latest/miniconda.html
[conda install]: https://conda.io/projects/conda/en/latest/user-guide/install/index.html
