"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""

import os
import sys
import argparse


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

    args = parser.parse_args()

    arch = determine_architecture(args.architecture)
    variant = determine_variant(args.variant, arch)

    if args.fetch_irtss:
        dummy_config = Config(arch, variant, "a", "b")
        if not os.path.isdir(dummy_config.irtss_release_path):
            dummy_config.download_irtss()
            print("IRTSS release downloaded.")
        else:
            print("IRTSS release already exists.")
        sys.exit(0)

    source = args.input
    if source.endswith("/"):  # Trim away trailing slashes
        source = source.rsplit("/", 1)[0]

    if source is None:
        print("Currently, input files must be passed explicitly.")
        sys.exit(1)
        # TODO more cases, like a config file etc.

    if args.output is None:
        out = source.rsplit(".rs", 1)[0]
        if out == source:  # For crates
            out = out + ".out"
        # Make output land in current working directory
        out = os.path.basename(out)

    else:
        out = args.output

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
