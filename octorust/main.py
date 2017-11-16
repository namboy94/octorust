#!/usr/bin/env python
"""
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octorust.

octorust is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octorust is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octorust.  If not, see <http://www.gnu.org/licenses/>.
"""

import os
import sys
from octorust.util.runner import run_executable
from octorust.util.cli_parse import generate_config
from octorust.dependencies.irtss import get_irtss_release
from octorust.recipes.c import compile_c
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
        get_irtss_release(config.irtss_release_path, config.build_version,
                          config.arch, config.variant)

    print(config.irtss_release_path)
    print(os.path.isdir(config.irtss_release_path))
    if not os.path.isdir(config.irtss_release_path):
        print("No matching IRTSS release installed. Please use the " +
              "--fetch-irtss option to download the release.")
        sys.exit(1)

    if "compile_rustc" in config.mode:
        compile_using_rustc(config)

    elif "compile_cargo" in config.mode:
        compile_using_cargo(config)

    elif "compile_c" in config.mode:
        compile_c(config)

    if "run" in config.mode and config.mode[0].startswith("compile"):
        run_executable(config.output, config.arch)


if __name__ == "__main__":  # Main Entry Point
    main()
