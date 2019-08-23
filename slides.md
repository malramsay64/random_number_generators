# Random Number Generators

::: notes

Before we went on a break,
Paul showed us Perlin noise,
a type of random field which is useful for generating terrain.
This raised the question,
how do you generated random numbers?
So I am taking the opportunity to go into far too much detail
about random numbers.
Before I do,
How would you define a random number?

- Sequence which can't be predicted
- numbers which adhere to a distribution
- sequence of numbers with no autocorrelation

## Pseudo Random Numbers

::: notes

When we talk about random numbers
we are typically referring to Pseudo Random Numbers.
Much of the time when say we want random numbers,
like a Monte Carlo simulation,
we actually want numbers from a distribution
and true randomness is a bug.
It's much nicer when we get the same result
when re-running an experiment,
than different results each time.

A Pseudo Random Number Generator (PRNG)
is an algorithm for taking a known state,
and performing some transformation
on that state to get the next state.
So what do I mean by this?

Lets take the simplest family of pseudo random number generators,
known as linear congruential generators.
These take a number $X$,
and generate the next number in the sequence
using the relation

$$ X_{n+1} = (aX_n + c) \mod m $$

With a 'reasonable' choice of $a$ and $c$ you can get
a 'reasonable' random number generator.
However, you *never* want to be using a LCG.

Why?
Let me tell you a story about RANDU

## RANDU

 :: notes

This is a random number generator with the relation

$$ X_{n+1} = $65539 X_n$ mod 2^{31} $$

It was developed by IBM in the early 60s
and used for being simple to calculate,
65539 is $2^16 + 3$,
making the calculation of the next random number super simple,
basically ignoring the multiplication,
and doing raw bit operations.

However, and this is the fun bit,
when generating points in 3d space,
x,y, and z triples,
all the generated points lie on one of 15 planes,
which is fairly clearly an indication
not a good thing.

Now this was known about in 1963,
however, it was still widely used as a PRNG
throughout the 70s,
widely used within FORTRAN,
which means it took a long time to deprecate,
still found in a [manual from 1999](http://h30266.www3.hpe.com/odl/unix/progtool/cf95au56/dflrm.htmhttp://h30266.www3.hpe.com/odl/unix/progtool/cf95au56/dflrm.htm)

## What do we use currently?

C++ 11
: Linear Congruential Generator
: Mersenne Twister

Fortran
: Linear Congruential Generator (RAND in Fortran 77)
: Xorshift (RANDOM in Fortran 95)

Python
: Mersenne Twister
: Permutation Congruential Generator (Numpy 1.17)

Julia
: Mersenne Twister
: Permutation Congruential Generator (RandomNumbers)
: Xorshift (RandomNumbers)

Rust
: Permutation Congruential Generator (rand)
: Xorshift (rand)

::: notes


## What makes a good random number generator

RANDU is really, really bad
what about some tests which identify
what is a good PRNG?

- tests of randomness
- long sequence length
    - only want to take $\sqrt{2]$ of the entire sequence
    - come across issues of replacement

## Are they any good?

::: notes

- In short yes

However, one measure of randomness
is that numbers are unpredictable.
In all these random number generators,
it is possible to work out the state
from the sequence of numbers.

## CSPRNG

::: notes

- Still take an unknown state and modify it
- Only now it is not possible to predict the next state
    - Used in cryptography
        - two factor authentication tokens
    - games

## What if we do want 'true' randomness

::: notes

- Computers are terrible at randomness
- However we can generate some randomness
    - taking a measurement to many decimal places the additional decimal places are
      typically considered random
    - This is how a computer generates truly randomness

- This is a slow process
- computers are fast

- Random.org
    - atmospheric noise

- In practice, truly random number generators are used to generate the state of a CSPRNG
  which is then iterated on for a period of time before being re-seeded from the random
  number generator

## Cloudflare

::: notes

 - Web security is their business
    - Use random number generators extensively
    - Disastrous implications if random numbers are not random

- RANDU was found to not be random
    - Catastrophic implications if their random number generator is not as random as
      they think it is.
    - As a 'backup' source of randomness use lava lamps
    - Nothing quite like the real world for messing things up.
