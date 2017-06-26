"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
import json
from shutil import copyfile
from subprocess import Popen
from octorust.config import Config
from octorust.recipes.shared import cleanup


def compile_rust_static_library(config: Config):
    """
    Compiles a rust file or crate as a static '.a' library. In case this fails,
    the program will exit.
    :param config: The previously determined configuration 
    :return: None
    """
    if os.path.isdir(config.source):
        compile_cargo(config)
    elif os.path.isfile(config.source):
        compile_rustc(config)
    else:
        print(config.source + " does not exist.")
        sys.exit(1)


def compile_rustc(config: Config):
    """
    Compiles a single rust file
    :param config: The compilation configuration
    :return: None
    """

    command = ["rustc", "--crate-type", "staticlib"]

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


def compile_cargo(config: Config):
    """
    Compiles a rust crate project using cargo and automatically
    includes octolib
    :param config: The compilation configuration
    :return: None
    """

    crate_name = os.path.basename(config.source)
    if not crate_name:
        crate_name = os.path.dirname(config.source)
    print(crate_name)

    current = os.getcwd()
    os.chdir(config.source)

    command = ["cargo"]
    output = ""

    if config.arch == "x86guest":
        command += ["build", "--target", "i686-unknown-linux-gnu"]
        output = "target/i686-unknown-linux-gnu/debug/lib" + crate_name + ".a"
    elif config.arch == "x64guest":
        command += ["build"]
        output = "target/debug/lib" + crate_name + ".a"
    elif config.arch == "leon":
        generate_leon_specification()
        command += ["rustc", "--target", "leon", "--", "-C", "link-dead-code"]
        output = "target/leon/debug/lib" + crate_name + ".a"

    generate_cargo_toml(config)
    Popen(command).wait()

    os.rename(output, os.path.join(current, "lib" + crate_name + ".a"))
    cleanup(["leon.json", "Cargo.toml", "target"])

    os.chdir(current)


def generate_cargo_toml(config: Config):
    """
    Generates a Cargo.toml file from the crate's OctoCargo.toml file
    :param config: The config to use while compiling
    :return: None
    """

    copyfile("OctoCargo.toml", "Cargo.toml")

    with open("Cargo.toml", 'a') as octocargo:
        octocargo.write(
            "core = { path = \"" + config.libcore + "\" }\n" +
            "octolib = { path = \"" + config.octolib + "\" }"
        )

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
