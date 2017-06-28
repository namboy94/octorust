"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
from subprocess import Popen

from octorust.linking.shared import cleanup
from octorust.util.config import Config


def compile_c_object(config: Config):
    """
    Compiles A C file that acts as a compatibility layer to OctoPOS
    :param config: The config that defines the compiler options
    :return: None
    """

    generate_c_dummy(config.c_file)

    command = [config.gcc]

    if config.arch.startswith("x"):

        if config.arch == "x86guest":

            command += [
                "-mfpmath=sse",
                "-msse2",
                "-m32",
                "-nostdinc",
                "-fno-asynchronous-unwind-tables",
                "-fno-stack-protector",
            ]

        elif config.arch == "x64native":

            command += [
                "-m64",
                "-fno-stack-protector",
                "-mno-red-zone",
                "-nodefaultlibs",
                "-nostdlib",
                "-mcx16",
                "-D__STDC_LIMIT_MACROS",
                "-O3",
                "-nostdinc",
                "-fno-asynchronous-unwind-tables",
                "-fno-stack-protector",
            ]

        command += [  # Same for x86guest and x64native
            "-I" + config.irtss_include,
            "-isystem",
            config.get_native_path("include"),
            "-D__OCTOPOS__",
            "-std=gnu11"
        ]

    elif config.arch == "leon":

        command += [
            "-mcpu=v8",
            "-O3",
            "-nostdinc",
            "-fno-asynchronous-unwind-tables",
            "-fno-stack-protector",
            "-I" + config.irtss_include,
            "-isystem",
            config.get_native_path("include"),
            "-D__OCTOPOS__",
            "-std=gnu99"

        ]

    command += ["-c", config.c_file, "-o", config.c_object]
    Popen(command).wait()
    cleanup([config.c_file])

    if not os.path.isfile(config.c_object):
        print("C Object did not compile. Can not continue.")
        sys.exit(1)


def generate_c_dummy(destination: str):
    """
    Writes a dummy C file that acts as 'glue' to get rust working with
    OctoPOS
    :param destination: The dummy file location
    :return: None
    """

    data = "#include <stdint.h>\n" \
           "#include <octopos.h>\n" \
           "void main_rust_ilet(uint8_t claim_t);\n" \
           "void main_ilet(uint8_t claim_t) {\n" \
           "    main_rust_ilet(claim_t);\n" \
           "}"

    with open(destination, 'w') as dummy:
        dummy.write(data)
