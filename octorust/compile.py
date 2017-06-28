#!/usr/bin/env python
"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
from octorust.cli_parse import generate_config
from octorust.recipes.rust import compile_rust_static_library
from octorust.recipes.c_object import compile_c_object
from octorust.recipes.octopos_link import link_app
from octorust.recipes.run import run_executable
from octorust.irtss import get_irtss_release


def main():
    """
    The main method of this program. It generates a config object
    and then attempts to compile the specified application
    :return: None
    """
    config = generate_config()

    if config.mode == "fetch_irtss":
        get_irtss_release(config.irtss_release_path,
                          config.arch, config.variant)

    elif config.mode == "compile_rustc":
        # TODO Compile Rust
        pass


    compile_rust_static_library(config)
    compile_c_object(config)
    link_app(config)

    if config.run:
        run_executable(config)


if __name__ == "__main__":  # Main Entry Point
    main()



