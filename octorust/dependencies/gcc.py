import os
import sys
from subprocess import check_output, Popen
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


def download_sparc_elf_gcc():
    """
    Downloads a sparc-elf-gcc if one can't be found in $PATH or in the
    .octorust directory
    :return: None
    """

    toolchain_dir = os.path.join(
        os.path.expanduser("~"), ".octorust", "toolchains")
    sparc_elf_root = os.path.join(toolchain_dir, "sparc-elf")
    sparc_elf_gcc = os.path.join(sparc_elf_root, "bin", "sparc-elf-gcc")

    if not check_if_in_path("sparc-elf-gcc") \
            and not os.path.isfile(sparc_elf_gcc):

        if not os.path.isdir(toolchain_dir):
            os.makedirs(toolchain_dir)

        gcc_url = "https://www4.cs.fau.de/invasic/tools/" \
                  "sparc-elf-7.1.0-x86_64.tar.bz2"
        Popen(["wget", gcc_url]).wait()
        Popen(["tar", "xjfv", "sparc-elf-7.1.0-x86_64.tar.bz2"]).wait()
        os.remove("sparc-elf-7.1.0-x86_64.tar.bz2")
        os.rename("sparc-elf-7.1.0", sparc_elf_root)
