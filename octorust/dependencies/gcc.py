import os
import sys
from subprocess import check_output
from octorust.dependencies.general import check_if_in_path


def determine_gcc(arch: str) -> str:
    """
    Determines the gcc command to use. Also checks if the gcc to use
    exists in a directory included in the $PATH variable.
    If a valid gcc could not be found, the program exits with a message
    indicating as such.
    :param arch: The architecture for which the gcc should be used
    :return: The gcc to use
    """

    gcc = {
        "x86guest": "gcc",
        "x64native": "gcc",
        "leon": "sparc-elf-gcc"}[arch]

    if not check_if_in_path(gcc):

        # Alternative sparc-elf-gcc in local .octorust directory
        sparc_elf_gcc = os.path.join(
            os.path.expanduser("~"),
            ".octorust", "toolchains", "sparc-elf", "bin", "sparc-elf-gcc")

        if arch == "leon" and os.path.isfile(sparc_elf_gcc):
            gcc = sparc_elf_gcc
        else:
            print("gcc '" + gcc + "' not in path. Please make sure that the"
                                  "gcc is set up correctly.")
            sys.exit(1)

    return gcc


def get_native_path(gcc: str, arch: str, target: str) -> str:
    """
    Queries gcc for a native object or include path
    :param gcc: The gcc command to use
    :param arch: The architecture for which to find the target
    :param target: The target whose path should be found
    :return: The path of the target
    """

    command = [gcc, "--print-file-name", target]
    if arch == "x86guest":
        command.append("-m32")

    return check_output(command).decode().strip()
