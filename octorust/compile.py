#!/usr/bin/env python
"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

from octorust.cli_parse import generate_config
from octorust.dependencies.irtss import get_irtss_release


def main():
    """
    The main method of this program. It generates a config object
    and then attempts to compile the specified application
    :return: None
    """
    config = generate_config()

    if "compile_rustc" in config.mode:
        # TODO Compile Rust
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
        # TODO Run
        pass


if __name__ == "__main__":  # Main Entry Point
    main()
