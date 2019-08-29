#! /usr/bin/env python
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright © 2019 Malcolm Ramsay <malramsay64@gmail.com>
#
# Distributed under terms of the MIT license.

"""Script to create the figure for the randu image."""

import sys
import textwrap

import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D


def main(infile):
    # Read in the input file
    #   Using a comma as the delimiter
    #   Skipping the first row
    data = np.loadtxt(infile, delimiter=",", skiprows=1)

    if data.shape[1] < 3:
        raise ValueError(
            "Input datafile doesn't contain enough columns, requires at least 3."
        )

    # Create a 3D Figure in matplotlib
    fig = plt.figure()
    ax = fig.add_subplot(111, projection="3d")

    # Plot first 3 columns of points with size of 1
    ax.scatter(data[:, 0], data[:, 1], data[:, 2], s=1)

    # Set the initial angle of the figure to view the planes
    ax.view_init(4, -124)

    fig.savefig("figures/randu.png", dpi=600)


if __name__ == "__main__":
    if len(sys.argv) == 2:
        infile = sys.argv[1]
        main(infile)
    else:
        print(
            textwrap.dedent(
                """
                    An input file is required as the only argument.
                    This can be generated by running the command:
                        random_number_generators randu --seed 0 --dimensions 3 --num_values 10000
                """
            )
        )