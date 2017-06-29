#!/usr/bin/env python
"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

from octorust.util.runner import run_executable
from octorust.util.cli_parse import generate_config
from octorust.dependencies.irtss import get_irtss_release
from octorust.recipes.rustc import compile_using_rustc
from octorust.recipes.cargo import compile_using_cargo


def main():
    """
    The main method of this program. It generates a config object
    and then attempts to compile the specified application
    :return: None
    """
    config = generate_config()

    if "fetch_irtss" in config.mode:
        get_irtss_release(config.irtss_release_path,
                          config.arch, config.variant)

    if "compile_rustc" in config.mode:
        compile_using_rustc(config)

    elif "compile_cargo" in config.mode:
        compile_using_cargo(config)

    if "run" in config.mode and config.mode[0].startswith("compile"):
        run_executable(config.output, config.arch)


if __name__ == "__main__":  # Main Entry Point
    main()
