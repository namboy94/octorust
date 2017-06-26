"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
import shutil
from subprocess import Popen
from octorust.config import Config
from octorust.recipes.shared import cleanup


def link_app(config: Config):

    command = [config.gcc, "-o", config.out]

    if config.arch.startswith("x"):

        if config.arch == "x86guest":
            command.append("-m32")
        elif config.arch == "x64native":
            command.append("-Wl,--build-id=none")

        command += [  # Shared arguments for x86guest and x64native
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

    elif config.arch == "leon":
        pass  # TODO

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
    # TODO PLEASE FIX ME!!!
    # TODO Find these dynamically, determined by architecture of course

    if config.arch == "x86guest":

        if target == "crti.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib32/crti.o"
        elif target == "crtbegin.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/32/crtbegin.o"
        elif target == "crtend.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/32/crtend.o"
        elif target == "crtn.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib32/crtn.o"

    elif config.arch == "x64native":

        if target == "crti.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib/crti.o"
        elif target == "crtbegin.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/crtbegin.o"
        elif target == "crtend.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/crtend.o"
        elif target == "crtn.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib/crtn.o"

    elif config.arch == "leon":

        if target == "crti.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib/crti.o"
        elif target == "crtbegin.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/crtbegin.o"
        elif target == "crtend.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/crtend.o"
        elif target == "crtn.o":
            return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib/crtn.o"


    return ""
