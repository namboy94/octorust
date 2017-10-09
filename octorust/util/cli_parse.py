"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import argparse
import os
import sys
from typing import List

from octorust.util.config import Config


# For accessing the parsed argparse.Namespace, ignore unresolved references
# noinspection PyUnresolvedReferences
def generate_config() -> Config:
    """
    Generates a Config object from the command line arguments
    :return: The generated Config object
    """
    args = parse_args()

    # Get parser results
    arch = determine_architecture(args.architecture)
    variant = determine_variant(args.variant, arch)
    source = args.input

    # Trim away trailing slashes
    if source is not None and source.endswith("/"):
        source = source.rsplit("/", 1)[0]

    mode = determine_mode(source, args.fetch_irtss, args.run)
    output = determine_output_path(args.output, source, mode)

    build_version = "current" if args.irtss_build_version is None\
        else args.irtss_build_version

    return Config(
        arch,
        variant,
        source, output,
        mode,
        args.keep,
        args.release,
        build_version
    )


def parse_args() -> argparse.Namespace:
    """
    Parses the command line arguments from the CLI
    :return: The parsed argparse Namespace
    """

    parser = argparse.ArgumentParser()

    parser.add_argument("-a", "--architecture",
                        help="The target architecture for the application. "
                             "May be one of (leon|x86guest|x64native). "
                             "Defaults to x86guest.")
    parser.add_argument("-v", "--variant",
                        help="Specifies the variant of the target, for example"
                             " 'generic' or '4t5c-chipit' etc."
                             "Defaults to an appropriate value for the "
                             "specified architecture.")
    parser.add_argument("input", nargs="?",
                        help="The input source file to compile")
    parser.add_argument("-o", "--output",
                        help="The output binary file. "
                             "Defaults to a name based on the input file")
    parser.add_argument("--fetch-irtss", action="store_true",
                        help="Can be used to dowload an IRTSS release. "
                             "Will use the specified architecture and variant")
    parser.add_argument("-r", "--run", action="store_true",
                        help="Executes the application after compilation")
    parser.add_argument("-k", "--keep", action="store_true",
                        help="Keeps the produced files. Helpful for debugging")
    parser.add_argument("--release", action="store_true",
                        help="Compiles Rust programs in release mode")
    parser.add_argument("-i", "--irtss-build-version",
                        help="Optionally sets the irtss build version")

    return parser.parse_args()


def determine_mode(source: str, fetch_irtss: bool, run: bool) -> List[str]:
    """
    Determines the mode in which the program will run
    :param source: The source file/directory location
    :param fetch_irtss: A flag that can be set to make the
                        application download an IRTSS release.
                        Will be ranked below any actual compilation job
    :param run: This flag can be set to automatically run a program upon
                completion
    :return: The resulting mode as a list of job strings.
    """

    mode = []

    if source is not None and source.endswith(".rs"):
        print("Compiling using rustc")
        mode.append("compile_rustc")

    elif source is not None and source.endswith(".c"):
        print("Compiling C file")
        mode.append("compile_c")

    elif source is not None and \
            os.path.isfile(os.path.join(source, "OctoCargo.toml")):
        print("Compiling using Cargo:")
        mode.append("compile_cargo")

    if fetch_irtss:
        print("Fetching IRTSS:")
        mode.append("fetch_irtss")

    if len(mode) == 0:  # Undefined modes can't be used
        print("Undefined Mode, please check your combination of arguments")
        sys.exit(1)

    if run and mode[0].startswith("compile"):
        mode.append("run")

    return mode


def determine_output_path(output_argument: str or None, source: str,
                          mode: List[str]) -> str or None:
    """
    Determines the file path for the compiled output file
    :param output_argument: The user-supplied output argument,
                            which may not be set
    :param source:          The source file/directory
    :param mode:            The previously established application mode
    :return:                The output file path,
                            or None if no source was provided
    """

    if source is None:
        return None

    else:
        output = output_argument

        if output_argument is None:

            if "compile_rustc" in mode:
                output = source.rsplit(".rs", 1)[0]
                output = os.path.basename(output)

            elif "compile_c" in mode:
                output = source.rsplit(".c", 1)[0]
                output = os.path.basename(output)

            elif "compile_cargo" in mode:
                # It's safe to use basename here since the trailing
                # forward slashes were already removed
                output = os.path.basename(source)

            else:  # Unsupported mode for compilation
                return None

        if output == source:
            output += ".out"

        return output


def determine_architecture(arch_param: str) -> str:
    """
    Determines the target architecture by either selecting the one provided as
    an argument or returning the default architecture (x86guest).
    If an unsupported architecture is provided, the program will exit.
    :param arch_param: The argument parameter for the architecture
    :return: The selected architecture
    """
    arch = arch_param if arch_param is not None else "x86guest"
    if arch not in ["x86guest", "x64native", "leon"]:
        print("Unsupported architecture '" + arch + ".")
        print("Only 'x86guest', 'x64native' and 'leon' are supported.")
        sys.exit(1)  # Exits the Program
    return arch


def determine_variant(variant_param: str, arch: str) -> str():
    """
    Determines the target variant by either using the provided variant
    or defaulting to an appropriate variant for the selected architecture
    :param variant_param: The variant argument parameters
    :param arch: The previously determined architecture
    :return: The selected variant
    """
    if variant_param is not None:
        return variant_param
    else:
        return {
            "x86guest": "generic",
            "x64native": "generic",
            "leon": "4t5c-nores-chipit-w-iotile"
        }[arch]
