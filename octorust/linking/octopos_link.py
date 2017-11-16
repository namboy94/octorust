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
from typing import List
from subprocess import Popen
from octorust.recipes.c_obj import compile_c_object
from octorust.util.config import Config


def link_app(config: Config, link_targets: List[str]):
    """
    Links the final application with the various libraries
    :param config: The configuration
    :param link_targets: The targets to link to IRTSS/OctoPOS
    :return: None
    """

    c_lib_objects = compile_c_helper_lib(config)

    command = [config.gcc, "-o", config.output]

    if config.arch == "x86guest":
        command.append("-m32")

    elif config.arch == "x64native":
        command.append("-Wl,--build-id=none")

    command += [  # Shared arguments for all architectures
        "-L" + config.irtss_lib,
        "-nostdlib",
        "-Wl,-T," + config.irtss_sections_x + ",--gc-sections",
        config.crti_o,
        config.crtbegin_o]

    command += link_targets + c_lib_objects

    command += [
        "-loctopos",
        "-lc++",
        "-lcsubset",
        "-loctopos",
        "-lgcc",
        config.crtend_o,
        config.crtn_o,
        "-lotail",
        "-static"
    ]

    for part in command:
        print(part, end=" ")
    print()
    Popen(command).wait()

    if not os.path.isfile(config.output):
        print("Compilation failed.")
        sys.exit(1)

    if config.arch == "x64native":  # Make x64native executable
        Popen(
            ["objcopy", "-I", "elf64-x86-64", "-O", "elf32-i386",
             config.output]
        ).wait()

    for c_object in c_lib_objects:
        os.remove(c_object)


def compile_c_helper_lib(config: Config) -> List[str]:
    """
    Compiles the C Helper Library

    :param config: The configuration
    :return: A list of object file locations generated while compiling the
             helper library
    """

    objects = []

    c_lib = os.path.join(config.octolib, "c")

    for c_file in os.listdir(c_lib):
        if c_file.endswith(".c"):
            c_file_path = os.path.join(c_lib, c_file)
            objects.append(compile_c_object(config, c_file_path))

    return objects
