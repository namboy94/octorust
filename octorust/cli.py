"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import argparse
import sys

from octorust.config import Config


def parse_args() -> Config:
    """
    Parses the arguments for the CLI
    :return: A configuration object for the provided arguments
    """

    parser = argparse.ArgumentParser()

    parser.add_argument("-a", "--architecture",
                        help="The target architecture for the application. "
                             "May be one of (leon|x86guest|x64native). "
                             "Defaults to x86guest.")
    parser.add_argument("-v", "--variant",
                        help="Specifies the variant of the target, for example "
                             "'generic' or '4t5c-chipit' etc."
                             "Defaults to an appropriate value for the "
                             "specified architecture.")
    parser.add_argument("input", nargs="?",
                        help="The input source file to compile")
    parser.add_argument("-o", "--output",
                        help="The output binary file. "
                             "Defaults to a name based on the input file")

    args = parser.parse_args()

    arch = determine_architecture(args.architecture)
    variant = determine_variant(args.variant, arch)
    source = args.input

    if source is None:
        print("Currently, input files must be passed explicitly.")
        sys.exit(1)
        # TODO more cases, like a config file etc.

    out = args.output if args.output is not None else source.rsplit(".rs", 1)[0]

    return Config(arch, variant, source, out)


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
