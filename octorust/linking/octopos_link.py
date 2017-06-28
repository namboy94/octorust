"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
from typing import List
from subprocess import Popen
from octorust.util.config import Config


def link_app(config: Config, link_targets: List[str]):

    command = [config.gcc, "-o", config.output]

    if config.arch == "x86guest":
        command.append("-m32")

    elif config.arch == "x64native":
        command.append("-Wl,--build-id=none")

    command += [  # Shared arguments for all architectures
        "-L" + config.irtss_lib,
        "-nostdlib",
        "-Wl,-T," + config.irtss_sections_x,
        config.crti_o,
        config.crtbegin_o]

    command += link_targets

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

    Popen(command).wait()

    if not os.path.isfile(config.output):
        print("Compilation failed.")
        sys.exit(1)

    if config.arch == "x64native":  # Make x64native executable
        Popen(
            ["objcopy", "-I", "elf64-x86-64", "-O", "elf32-i386",
             config.output]
        ).wait()
