import os
import sys
from subprocess import Popen
from octorust.util.config import Config
from octorust.util.files import cleanup
from octorust.linking.octopos_link import link_app


def compile_c(config: Config):
    """
    Compiles a C file and links it to OctoPOS
    :param config: The configuration to use while compiling
    :return: None
    """
    object_file = compile_c_object(config, config.source)
    link_app(config, [object_file])
    cleanup([object_file], config)


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

        command += ["-mfpmath=sse", "-msse2", "-m32"]

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

    print(command)
    Popen(command).wait()

    if not os.path.isfile(object_file):
        print("C Object did not compile. Can not continue.")
        sys.exit(1)

    return object_file
