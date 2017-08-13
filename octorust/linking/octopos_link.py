"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
from typing import List
from subprocess import Popen
from octorust.util.config import Config
#from octorust.recipes.c import compile_c_object


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
        c_file_path = os.path.join(c_lib, c_file)
        objects.append(compile_c_object(config, c_file_path))

    return objects


# TODO Remove this, this is code duplication!
def compile_c_object(config: Config, target: str) -> str:
    """
    Compiles a C object file which can be linked to OctoPOS
    :param config: The configuration to use while compiling
    :param target: The target file to compile
    :return: The path to the generated object file
    """

    gnu_std = "99" if config.arch == "leon" else "11"
    object_file = target.rsplit(".c", 1)[0] + ".o"

    command = [config.gcc]

    if config.arch == "x86guest":

        command += ["-mfpmath=sse", "-msse2", "-m32", "-O3"]

    elif config.arch == "x64native":

        command += [
            "-m64",
            "-fno-stack-protector",
            "-mno-red-zone",
            "-nodefaultlibs",
            "-nostdlib",
            "-mcx16",
            "-D__STDC_LIMIT_MACROS",
            "-O3"
        ]

    elif config.arch == "leon":
        command += ["-mcpu=v8", "-O3"]

    command += [  # Same for all architectures
        "-nostdinc",
        "-fno-asynchronous-unwind-tables",
        "-fno-stack-protector",
        "-I" + config.irtss_include,
        "-isystem",
        config.gcc_include,
        "-D__OCTOPOS__",
        "-std=gnu" + gnu_std,
        "-c", target, "-o", object_file
    ]

    for part in command:
        print(part, end=" ")
    print()
    Popen(command).wait()

    if not os.path.isfile(object_file):
        print("C Object did not compile. Can not continue.")
        sys.exit(1)

    return object_file
