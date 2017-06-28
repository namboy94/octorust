#!/usr/bin/env python
"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

from octorust.util.runner import run_executable
from octorust.util.cli_parse import generate_config
from octorust.dependencies.irtss import get_irtss_release
from octorust.recipes.rustc import compile_using_rustc


def main():
    """
    The main method of this program. It generates a config object
    and then attempts to compile the specified application
    :return: None
    """
    config = generate_config()

    if "compile_rustc" in config.mode:
        compile_using_rustc(config)
        pass

    elif "compile_cargo" in config.mode:
        # TODO Compile Cargo
        pass

    elif ["fetch_irtss"] in config.mode:
        get_irtss_release(config.irtss_release_path,
                          config.arch, config.variant)

    else:
        print("Invalid Mode " + config.mode + ".")
        print("Please double-check your input")

    if "run" in config.mode:
        run_executable(config.output, config.arch)
        pass


if __name__ == "__main__":  # Main Entry Point
    main()
