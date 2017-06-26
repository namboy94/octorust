"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
from subprocess import Popen, check_output
from octorust.config import Config
from octorust.recipes.shared import cleanup


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
        get_native_object("crti.o", config),
        get_native_object("crtbegin.o", config),
        config.c_object,
        config.rust_static_lib,
        "-loctopos",
        "-lc++",
        "-lcsubset",
        "-loctopos",
        "-lgcc",
        get_native_object("crtend.o", config),
        get_native_object("crtn.o", config),
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


def get_native_object(target: str, config: Config) -> str:

    command = [config.gcc, "--print-file-name", target]
    if config.arch == "x86guest":
        command.append("-m32")

    return check_output(command).decode().strip()
