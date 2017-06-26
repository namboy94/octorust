"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
from subprocess import Popen
from octorust.config import Config
from octorust.recipes.shared import cleanup


def link_app(config: Config):

    command = [config.gcc, "-o", config.out]

    if config.arch == "x86guest":
        command.append("-m32")  # 32 bit
        command.append("-L" + config.irtss_lib)
        command.append("-nostdlib")
        command.append("-Wl,-T," + config.irtss_sections_x)
        command.append(get_native_object("crti.o"))
        command.append(get_native_object("crtbegin.o"))
        command.append(config.c_object)
        command.append(config.rust_static_lib)
        command.append("-loctopos")
        command.append("-lc++")
        command.append("-lcsubset")
        command.append("-loctopos")
        command.append("-lgcc")
        command.append(get_native_object("crtend.o"))
        command.append(get_native_object("crtn.o"))
        command.append("-lotail")
        command.append("-static")

    elif config.arch == "x64native":
        pass  # TODO
    elif config.arch == "leon":
        pass  # TODO

    Popen(command).wait()
    cleanup([config.c_object, config.rust_static_lib])

    if not os.path.isfile(config.out):
        print("Compilation failed.")
        sys.exit(1)


def get_native_object(target: str) -> str:
    # TODO Find these dynamically, determined by architecture of course
    if target == "crti.o":
        return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib32/crti.o"
    elif target == "crtbegin.o":
        return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/32/crtbegin.o"
    elif target == "crtend.o":
        return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/32/crtend.o"
    elif target == "crtn.o":
        return "/usr/lib/gcc/x86_64-pc-linux-gnu/7.1.1/../../../../lib32/crtn.o"

