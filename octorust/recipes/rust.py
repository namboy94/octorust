"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
import json
from subprocess import Popen
from octorust.config import Config
from octorust.recipes.shared import cleanup


def compile_rust_static_library(config: Config):
    """
    Compiles a rust file as a static '.a' library. In case this fails,
    the program will exit.
    :param config: The previously determined configuration 
    :return: None
    """

    command = ["rustc"]

    if config.arch == "x86guest":
        command += ["--target", "i686-unknown-linux-gnu"]
    elif config.arch == "x64guest":
        command += []
    elif config.arch == "leon":
        generate_leon_specification()
        command += ["--target", "leon"]

    command.append(config.source)
    Popen(command).wait()

    cleanup(["leon.json"])

    if not os.path.isfile(config.rust_static_lib):
        print("Something went wrong while compiling the rust file.")
        sys.exit(1)


def generate_leon_specification():
    """
    Generates a rust target specification file for the SPARC LEON architecture
    The specification file assumes a gcc cross-compiler for the architecture
    in the path called 'sparc-leon-linux-uclibc-gcc', which is one of the
    sample targets when using crosstool-ng.
    The specification file is saved as leon.json
    :return: None
    """

    leon_spec = {
        "arch": "sparc",
        "data-layout": "E-m:e-p:32:32-i64:64-f128:64-n32-S64",
        "executables": True,
        "llvm-target": "sparc",
        "os": "none",
        "panic-strategy": "abort",
        "target-endian": "big",
        "target-pointer-width": "32",
        "linker-flavor": "ld",
        "linker": "sparc-leon-linux-uclibc-gcc",
        "link-args": [
            "-nostartfiles"
        ]
    }

    with open("leon.json", 'w') as spec_file:
        json.dump(leon_spec, spec_file)
