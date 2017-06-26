#!/usr/bin/env python3
"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""


from octorust.cli import parse_args
from octorust.recipes.rust import compile_rust_static_library
from octorust.recipes.c_object import compile_c_object
from octorust.recipes.octopos_link import link_app


if __name__ == "__main__":  # Main Entry Point

    config = parse_args()
    compile_rust_static_library(config)
    compile_c_object(config)
    link_app(config)
