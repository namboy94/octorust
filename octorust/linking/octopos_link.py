"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
from subprocess import Popen

from octorust.linking.shared import cleanup
from octorust.util.config import Config


def link_app(config: Config):

    command = [config.gcc, "-o", config.out]

    if config.arch == "x86guest":
        command.append("-m32")
    elif config.arch == "x64native":
        command.append("-Wl,--build-id=none")

    command += [  # Shared arguments for all architectures
        "-L" + config.irtss_lib,
        "-nostdlib",
        "-Wl,-T," + config.irtss_sections_x,
        config.get_native_path("crti.o"),
        config.get_native_path("crtbegin.o"),
        config.c_object,
        config.rust_static_lib,
        "-loctopos",
        "-lc++",
        "-lcsubset",
        "-loctopos",
        "-lgcc",
        config.get_native_path("crtend.o"),
        config.get_native_path("crtn.o"),
        "-lotail",
        "-static"
    ]

    Popen(command).wait()
    cleanup([config.c_object, config.rust_static_lib])

    if not os.path.isfile(config.out):
        print("Compilation failed.")
        sys.exit(1)

    if config.arch == "x64native":  # Make x64native executable with qemu
        Popen(
            ["objcopy", "-I", "elf64-x86-64", "-O", "elf32-i386", config.out]
        ).wait()
